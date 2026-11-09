//! Authentication: credential checks with account lock-out, session issuance, refresh-token
//! rotation with reuse detection, and resolving an access token back into an [`Identity`].

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tracing::{info, instrument, warn};

use super::user::{PasswordHasher, UserPersistence};
use crate::{
    app_error::{AppError, AppResult},
    entities::{
        session::{Identity, RefreshToken, Session},
        user::{Credentials, User},
    },
    time::{iso_in, now_iso},
};

/// Issues and verifies the short-lived, self-contained access tokens.
pub trait AccessTokenService: Send + Sync {
    fn issue_access_token(&self, user_id: &str) -> AppResult<String>;
    /// The user id, if the token is authentic and not expired.
    fn verify_access_token(&self, token: &str) -> Option<String>;
}

/// Creates opaque refresh tokens and the fingerprints under which they are stored.
pub trait RefreshTokenFactory: Send + Sync {
    fn generate_refresh_token(&self) -> AppResult<String>;
    fn fingerprint(&self, token: &str) -> String;
}

#[async_trait]
pub trait SessionPersistence: Send + Sync {
    async fn store_refresh_token(
        &self,
        user_id: &str,
        fingerprint: &str,
        expires_at: &str,
    ) -> AppResult<()>;
    async fn find_refresh_token(&self, fingerprint: &str) -> AppResult<Option<RefreshToken>>;
    /// Atomically marks an active token as used. `false` if it was already used.
    async fn consume_refresh_token(&self, id: &str) -> AppResult<bool>;
    async fn revoke_refresh_token(&self, fingerprint: &str) -> AppResult<()>;
    async fn revoke_user_refresh_tokens(&self, user_id: &str) -> AppResult<()>;
    async fn delete_refresh_tokens_expired_before(&self, instant: &str) -> AppResult<u64>;
}

/// Tunables of the authentication rules.
#[derive(Debug, Clone)]
pub struct AuthPolicy {
    pub max_login_attempts: i64,
    pub lockout_duration: Duration,
    pub refresh_token_ttl: Duration,
}

const INVALID_CREDENTIALS: &str = "Invalid email or password";

#[derive(Clone)]
pub struct AuthUseCases {
    users: Arc<dyn UserPersistence>,
    sessions: Arc<dyn SessionPersistence>,
    hasher: Arc<dyn PasswordHasher>,
    access_tokens: Arc<dyn AccessTokenService>,
    refresh_tokens: Arc<dyn RefreshTokenFactory>,
    policy: AuthPolicy,
}

impl AuthUseCases {
    pub fn new(
        users: Arc<dyn UserPersistence>,
        sessions: Arc<dyn SessionPersistence>,
        hasher: Arc<dyn PasswordHasher>,
        access_tokens: Arc<dyn AccessTokenService>,
        refresh_tokens: Arc<dyn RefreshTokenFactory>,
        policy: AuthPolicy,
    ) -> Self {
        Self {
            users,
            sessions,
            hasher,
            access_tokens,
            refresh_tokens,
            policy,
        }
    }

    #[instrument(skip_all)]
    pub async fn login(&self, credentials: Credentials) -> AppResult<(User, Session)> {
        credentials.validate()?;
        let Some(user) = self
            .users
            .find_user_by_email(credentials.email.trim())
            .await?
        else {
            self.hasher
                .simulate_verification(&credentials.password)
                .await?;
            return Err(AppError::Unauthorized(INVALID_CREDENTIALS.into()));
        };

        let now = now_iso();
        if user.is_locked_at(&now) {
            let minutes = self.policy.lockout_duration.as_secs().div_ceil(60);
            return Err(AppError::Forbidden(format!(
                "Account locked due to too many failed attempts. Try again in {minutes} minutes."
            )));
        }

        if !self
            .hasher
            .verify_password(&credentials.password, &user.password_hash)
            .await?
        {
            self.register_failure(&user, &now).await?;
            return Err(AppError::Unauthorized(INVALID_CREDENTIALS.into()));
        }

        if user.failed_login_attempts > 0 || user.locked_until.is_some() {
            self.users.reset_failed_logins(&user.id).await?;
        }

        let session = self.start_session(&user.id).await?;
        Ok((user, session))
    }

    async fn register_failure(&self, user: &User, now: &str) -> AppResult<()> {
        // A lock that has already expired starts a fresh count.
        let previous = if user
            .locked_until
            .as_deref()
            .is_some_and(|until| until <= now)
        {
            0
        } else {
            user.failed_login_attempts
        };
        let attempts = previous + 1;
        let locked_until = (attempts >= self.policy.max_login_attempts)
            .then(|| iso_in(self.policy.lockout_duration));

        match &locked_until {
            Some(until) => warn!(user_id = %user.id, %until, "account locked after failed logins"),
            None => info!(user_id = %user.id, attempts, "failed login"),
        }
        self.users
            .record_failed_login(&user.id, attempts, locked_until.as_deref())
            .await
    }

    async fn start_session(&self, user_id: &str) -> AppResult<Session> {
        let access_token = self.access_tokens.issue_access_token(user_id)?;
        let refresh_token = self.refresh_tokens.generate_refresh_token()?;
        let expires_at = iso_in(self.policy.refresh_token_ttl);
        self.sessions
            .store_refresh_token(
                user_id,
                &self.refresh_tokens.fingerprint(&refresh_token),
                &expires_at,
            )
            .await?;
        Ok(Session {
            access_token,
            refresh_token,
        })
    }

    /// Rotates a refresh token: the presented token is consumed and a new session issued.
    /// Presenting an already-used token revokes every session of its owner (likely theft).
    /// Several candidates may be given (browsers can send stale cookies); the first known wins.
    #[instrument(skip_all)]
    pub async fn refresh(&self, candidates: &[&str]) -> AppResult<(User, Session)> {
        let mut found = None;
        for candidate in candidates {
            let fingerprint = self.refresh_tokens.fingerprint(candidate);
            if let Some(token) = self.sessions.find_refresh_token(&fingerprint).await? {
                found = Some(token);
                break;
            }
        }
        let token =
            found.ok_or_else(|| AppError::Unauthorized("Refresh token not found".into()))?;

        if token.revoked {
            self.sessions
                .revoke_user_refresh_tokens(&token.user_id)
                .await?;
            warn!(user_id = %token.user_id, "refresh token reuse detected; all sessions revoked");
            return Err(AppError::Unauthorized(
                "Token reuse detected — all sessions revoked".into(),
            ));
        }
        if token.expires_at.as_str() <= now_iso().as_str() {
            return Err(AppError::Unauthorized("Refresh token has expired".into()));
        }
        if !self.sessions.consume_refresh_token(&token.id).await? {
            return Err(AppError::Unauthorized(
                "Refresh token was already used".into(),
            ));
        }

        let user = self
            .users
            .find_user_by_id(&token.user_id)
            .await?
            .ok_or_else(AppError::unauthenticated)?;
        let session = self.start_session(&user.id).await?;
        Ok((user, session))
    }

    /// Ends sessions: all of the user's when known, otherwise just the presented tokens.
    pub async fn logout(&self, user_id: Option<&str>, refresh_tokens: &[&str]) -> AppResult<()> {
        if let Some(user_id) = user_id {
            self.sessions.revoke_user_refresh_tokens(user_id).await?;
        }
        for token in refresh_tokens {
            self.sessions
                .revoke_refresh_token(&self.refresh_tokens.fingerprint(token))
                .await?;
        }
        Ok(())
    }

    /// Resolves an access token. `None` when it is invalid, expired, or the account is gone.
    pub async fn authenticate(&self, access_token: &str) -> AppResult<Option<Identity>> {
        let Some(user_id) = self.access_tokens.verify_access_token(access_token) else {
            return Ok(None);
        };
        Ok(self
            .users
            .find_user_by_id(&user_id)
            .await?
            .map(|u| Identity {
                user_id: u.id,
                email: u.email,
            }))
    }

    /// Housekeeping: forgets refresh tokens that can no longer be used.
    pub async fn purge_expired_sessions(&self) -> AppResult<u64> {
        self.sessions
            .delete_refresh_tokens_expired_before(&now_iso())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::user::NewUser,
        use_cases::{
            fakes::{FakeHasher, FakeTokens, InMemory},
            user::UserUseCases,
        },
    };

    async fn setup() -> (Arc<InMemory>, AuthUseCases) {
        let store = Arc::new(InMemory::default());
        UserUseCases::new(store.clone(), Arc::new(FakeHasher))
            .register(NewUser {
                name: "Ana".into(),
                email: "ana@example.com".into(),
                password: "password1".into(),
            })
            .await
            .unwrap();
        let tokens = Arc::new(FakeTokens::default());
        let auth = AuthUseCases::new(
            store.clone(),
            store.clone(),
            Arc::new(FakeHasher),
            tokens.clone(),
            tokens,
            AuthPolicy {
                max_login_attempts: 3,
                lockout_duration: Duration::from_secs(900),
                refresh_token_ttl: Duration::from_secs(3600),
            },
        );
        (store, auth)
    }

    fn creds(password: &str) -> Credentials {
        Credentials {
            email: "ana@example.com".into(),
            password: password.into(),
        }
    }

    #[tokio::test]
    async fn login_and_authenticate() {
        let (_, auth) = setup().await;
        let (user, session) = auth.login(creds("password1")).await.unwrap();
        let identity = auth
            .authenticate(&session.access_token)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(identity.user_id, user.id);
        assert!(auth.authenticate("forged").await.unwrap().is_none());

        let unknown = Credentials {
            email: "nobody@example.com".into(),
            password: "x".into(),
        };
        assert!(matches!(
            auth.login(unknown).await,
            Err(AppError::Unauthorized(_))
        ));
    }

    #[tokio::test]
    async fn lockout_after_repeated_failures() {
        let (store, auth) = setup().await;
        for _ in 0..3 {
            assert!(matches!(
                auth.login(creds("wrong")).await,
                Err(AppError::Unauthorized(_))
            ));
        }
        assert!(
            matches!(auth.login(creds("password1")).await, Err(AppError::Forbidden(m)) if m.contains("15 minutes"))
        );

        // Once the lock has expired, the right password works and resets the counter.
        store.expire_locks();
        auth.login(creds("password1")).await.unwrap();
        let user = store.user_by_email("ana@example.com").unwrap();
        assert_eq!((user.failed_login_attempts, user.locked_until), (0, None));
    }

    #[tokio::test]
    async fn refresh_rotates_and_detects_reuse() {
        let (_, auth) = setup().await;
        let (_, first) = auth.login(creds("password1")).await.unwrap();

        let (_, second) = auth
            .refresh(&["unknown", &first.refresh_token])
            .await
            .unwrap();
        assert_ne!(first.refresh_token, second.refresh_token);

        // Replaying the consumed token revokes everything, including the rotated one.
        let err = auth.refresh(&[&first.refresh_token]).await.unwrap_err();
        assert!(matches!(err, AppError::Unauthorized(m) if m.contains("reuse")));
        assert!(auth.refresh(&[&second.refresh_token]).await.is_err());
    }

    #[tokio::test]
    async fn logout_revokes_presented_tokens() {
        let (_, auth) = setup().await;
        let (_, session) = auth.login(creds("password1")).await.unwrap();
        auth.logout(None, &[&session.refresh_token]).await.unwrap();
        assert!(auth.refresh(&[&session.refresh_token]).await.is_err());
    }
}
