//! bcrypt password hashing, compatible with the `$2a$`/`$2y$` hashes of the previous backend.
//! bcrypt is deliberately slow, so it always runs on the blocking thread pool.

use std::sync::OnceLock;

use async_trait::async_trait;

use crate::{
    app_error::{AppError, AppResult},
    use_cases::user::PasswordHasher,
};

pub struct BcryptPasswordHasher {
    cost: u32,
}

impl BcryptPasswordHasher {
    pub fn new(cost: u32) -> Self {
        Self { cost }
    }
}

#[async_trait]
impl PasswordHasher for BcryptPasswordHasher {
    async fn hash_password(&self, password: &str) -> AppResult<String> {
        let (password, cost) = (password.to_owned(), self.cost);
        tokio::task::spawn_blocking(move || bcrypt::hash(password, cost))
            .await
            .map_err(AppError::internal)?
            .map_err(|_| AppError::Internal("Password hashing failed.".into()))
    }

    async fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let (password, hash) = (password.to_owned(), hash.to_owned());
        tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash).unwrap_or(false))
            .await
            .map_err(AppError::internal)
    }

    async fn simulate_verification(&self, password: &str) -> AppResult<()> {
        static DUMMY: OnceLock<String> = OnceLock::new();
        let (password, cost) = (password.to_owned(), self.cost);
        tokio::task::spawn_blocking(move || {
            let hash = DUMMY
                .get_or_init(|| bcrypt::hash("cocookies-dummy-password", cost).unwrap_or_default());
            let _ = bcrypt::verify(password, hash);
        })
        .await
        .map_err(AppError::internal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn verifies_own_and_legacy_hashes() {
        let hasher = BcryptPasswordHasher::new(4);
        let h = hasher.hash_password("secret123").await.unwrap();
        assert!(hasher.verify_password("secret123", &h).await.unwrap());
        assert!(!hasher.verify_password("nope", &h).await.unwrap());
        // Hashes produced by PHP / Spring may use the $2y$ / $2a$ prefixes.
        for prefix in ["$2y$", "$2a$"] {
            let legacy = h.replacen("$2b$", prefix, 1);
            assert!(hasher.verify_password("secret123", &legacy).await.unwrap());
        }
        assert!(!hasher.verify_password("x", "not-a-hash").await.unwrap());
    }
}
