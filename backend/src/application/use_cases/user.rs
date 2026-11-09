//! Account management. Users can only see and change their own account.

use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::{
    app_error::{AppError, AppResult},
    entities::{
        session::Identity,
        user::{NewUser, PasswordChange, ProfileUpdate, User},
    },
};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn find_user_by_id(&self, id: &str) -> AppResult<Option<User>>;
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    /// Is `email` used by an account other than `except_id`?
    async fn user_email_taken(&self, email: &str, except_id: Option<&str>) -> AppResult<bool>;
    async fn create_user(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User>;
    async fn update_user_profile(&self, id: &str, name: &str, email: &str) -> AppResult<()>;
    async fn update_user_password(&self, id: &str, password_hash: &str) -> AppResult<()>;
    async fn record_failed_login(
        &self,
        id: &str,
        attempts: i64,
        locked_until: Option<&str>,
    ) -> AppResult<()>;
    async fn reset_failed_logins(&self, id: &str) -> AppResult<()>;
    /// Deletes the user and everything they own. Returns `false` if there was no such user.
    async fn delete_user(&self, id: &str) -> AppResult<bool>;
}

/// Slow, salted password hashing.
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash_password(&self, password: &str) -> AppResult<String>;
    async fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool>;
    /// Spends the same time as a real verification, so response timing does not reveal
    /// whether an account exists.
    async fn simulate_verification(&self, password: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct UserUseCases {
    persistence: Arc<dyn UserPersistence>,
    hasher: Arc<dyn PasswordHasher>,
}

impl UserUseCases {
    pub fn new(persistence: Arc<dyn UserPersistence>, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            persistence,
            hasher,
        }
    }

    #[instrument(skip_all)]
    pub async fn register(&self, new: NewUser) -> AppResult<User> {
        new.validate()?;
        let name = new.name.trim();
        let email = new.email.trim();
        if self.persistence.user_email_taken(email, None).await? {
            return Err(email_in_use(email));
        }

        let hash = self.hasher.hash_password(&new.password).await?;
        self.persistence
            .create_user(name, email, &hash)
            .await
            .map_err(|e| unique_as(e, || email_in_use(email)))
    }

    pub async fn get(&self, me: &Identity, id: &str) -> AppResult<User> {
        ensure_self(me, id, "You can only access your own account")?;
        self.load(id).await
    }

    #[instrument(skip(self, update), fields(user_id = %id))]
    pub async fn update(&self, me: &Identity, id: &str, update: ProfileUpdate) -> AppResult<User> {
        ensure_self(me, id, "You can only update your own account")?;
        update.validate()?;
        let email = update.email.trim();
        if self.persistence.user_email_taken(email, Some(id)).await? {
            return Err(email_in_use(email));
        }
        self.persistence
            .update_user_profile(id, update.name.trim(), email)
            .await
            .map_err(|e| unique_as(e, || email_in_use(email)))?;
        self.load(id).await
    }

    #[instrument(skip(self, change), fields(user_id = %id))]
    pub async fn change_password(
        &self,
        me: &Identity,
        id: &str,
        change: PasswordChange,
    ) -> AppResult<()> {
        ensure_self(me, id, "You can only change your own password")?;
        change.validate()?;
        let user = self.load(id).await?;

        let current = change
            .current_password
            .filter(|p| !p.is_empty())
            .ok_or_else(wrong_password)?;
        if !self
            .hasher
            .verify_password(&current, &user.password_hash)
            .await?
        {
            return Err(wrong_password());
        }

        let hash = self.hasher.hash_password(&change.new_password).await?;
        self.persistence.update_user_password(id, &hash).await
    }

    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn delete(&self, me: &Identity, id: &str) -> AppResult<()> {
        ensure_self(me, id, "You can only delete your own account")?;
        if !self.persistence.delete_user(id).await? {
            return Err(not_found(id));
        }
        Ok(())
    }

    async fn load(&self, id: &str) -> AppResult<User> {
        self.persistence
            .find_user_by_id(id)
            .await?
            .ok_or_else(|| not_found(id))
    }
}

fn ensure_self(me: &Identity, id: &str, message: &str) -> AppResult<()> {
    if me.owns(id) {
        Ok(())
    } else {
        Err(AppError::Forbidden(message.into()))
    }
}

/// Turns a persistence-level uniqueness violation into a meaningful error.
pub(crate) fn unique_as(err: AppError, conflict: impl FnOnce() -> AppError) -> AppError {
    match err {
        AppError::UniqueViolation => conflict(),
        other => other,
    }
}

fn not_found(id: &str) -> AppError {
    AppError::NotFound(format!("User not found with id: {id}"))
}

fn email_in_use(email: &str) -> AppError {
    AppError::Conflict(format!("Email already in use: {email}"))
}

fn wrong_password() -> AppError {
    AppError::Unauthorized("Current password is incorrect".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::fakes::{FakeHasher, InMemory};

    fn use_cases(store: &Arc<InMemory>) -> UserUseCases {
        UserUseCases::new(store.clone(), Arc::new(FakeHasher))
    }

    fn new_user(email: &str) -> NewUser {
        NewUser {
            name: "Ana".into(),
            email: email.into(),
            password: "password1".into(),
        }
    }

    #[tokio::test]
    async fn register_hashes_and_rejects_duplicates() {
        let store = Arc::new(InMemory::default());
        let users = use_cases(&store);

        let user = users.register(new_user(" ana@example.com ")).await.unwrap();
        assert_eq!(user.email, "ana@example.com");
        assert_eq!(user.password_hash, "hashed:password1");

        let err = users
            .register(new_user("ana@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Conflict(m) if m.contains("already in use")));

        let err = users.register(new_user("not-an-email")).await.unwrap_err();
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[tokio::test]
    async fn only_self_can_be_managed() {
        let store = Arc::new(InMemory::default());
        let users = use_cases(&store);
        let ana = users.register(new_user("ana@example.com")).await.unwrap();
        let bob = users.register(new_user("bob@example.com")).await.unwrap();
        let me = Identity {
            user_id: ana.id.clone(),
            email: ana.email.clone(),
        };

        assert!(matches!(
            users.get(&me, &bob.id).await,
            Err(AppError::Forbidden(_))
        ));
        assert!(matches!(
            users.delete(&me, &bob.id).await,
            Err(AppError::Forbidden(_))
        ));

        let taken = ProfileUpdate {
            name: "Ana".into(),
            email: "bob@example.com".into(),
        };
        assert!(matches!(
            users.update(&me, &ana.id, taken).await,
            Err(AppError::Conflict(_))
        ));

        let wrong = PasswordChange {
            current_password: Some("nope".into()),
            new_password: "newpassword".into(),
        };
        assert!(matches!(
            users.change_password(&me, &ana.id, wrong).await,
            Err(AppError::Unauthorized(_))
        ));
        let right = PasswordChange {
            current_password: Some("password1".into()),
            new_password: "newpassword".into(),
        };
        users.change_password(&me, &ana.id, right).await.unwrap();
        assert_eq!(
            users.get(&me, &ana.id).await.unwrap().password_hash,
            "hashed:newpassword"
        );
    }
}
