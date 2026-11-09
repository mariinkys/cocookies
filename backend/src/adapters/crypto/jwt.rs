//! Short-lived access tokens: HS256 JWTs whose subject is the user **id** (stable), not the
//! e-mail (which users can change).

use std::time::Duration;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    app_error::{AppError, AppResult},
    time::unix_now,
    use_cases::auth::AccessTokenService,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

pub struct JwtAccessTokens {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    ttl: Duration,
}

impl JwtAccessTokens {
    pub fn new(secret: &str, ttl: Duration) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 30;
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            validation,
            ttl,
        }
    }
}

impl AccessTokenService for JwtAccessTokens {
    fn issue_access_token(&self, user_id: &str) -> AppResult<String> {
        let now = unix_now();
        let claims = Claims {
            sub: user_id.to_owned(),
            iat: now,
            exp: now + self.ttl.as_secs() as i64,
        };
        jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &self.encoding)
            .map_err(AppError::internal)
    }

    fn verify_access_token(&self, token: &str) -> Option<String> {
        jsonwebtoken::decode::<Claims>(token, &self.decoding, &self.validation)
            .ok()
            .map(|data| data.claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_tamper() {
        let tokens = JwtAccessTokens::new(
            "a-very-long-secret-for-testing-purposes",
            Duration::from_secs(60),
        );
        let jwt = tokens.issue_access_token("user-1").unwrap();
        assert_eq!(tokens.verify_access_token(&jwt).as_deref(), Some("user-1"));

        let other = JwtAccessTokens::new(
            "another-very-long-secret-for-testing!!",
            Duration::from_secs(60),
        );
        assert_eq!(other.verify_access_token(&jwt), None);
        assert_eq!(tokens.verify_access_token("garbage"), None);
    }
}
