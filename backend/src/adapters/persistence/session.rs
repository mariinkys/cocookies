use async_trait::async_trait;
use sqlx::FromRow;

use super::{SqlitePersistence, new_id};
use crate::{
    app_error::AppResult, entities::session::RefreshToken, time::now_iso,
    use_cases::auth::SessionPersistence,
};

#[derive(FromRow)]
struct RefreshTokenDb {
    id: String,
    user_id: String,
    expires_at: String,
    revoked: bool,
}

impl From<RefreshTokenDb> for RefreshToken {
    fn from(db: RefreshTokenDb) -> Self {
        RefreshToken {
            id: db.id,
            user_id: db.user_id,
            expires_at: db.expires_at,
            revoked: db.revoked,
        }
    }
}

#[async_trait]
impl SessionPersistence for SqlitePersistence {
    async fn store_refresh_token(
        &self,
        user_id: &str,
        fingerprint: &str,
        expires_at: &str,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, revoked, created_at)
             VALUES (?, ?, ?, ?, 0, ?)",
        )
        .bind(new_id())
        .bind(user_id)
        .bind(fingerprint)
        .bind(expires_at)
        .bind(now_iso())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_refresh_token(&self, fingerprint: &str) -> AppResult<Option<RefreshToken>> {
        let row: Option<RefreshTokenDb> = sqlx::query_as(
            "SELECT id, user_id, expires_at, revoked FROM refresh_tokens WHERE token_hash = ?",
        )
        .bind(fingerprint)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(RefreshToken::from))
    }

    async fn consume_refresh_token(&self, id: &str) -> AppResult<bool> {
        let done =
            sqlx::query("UPDATE refresh_tokens SET revoked = 1 WHERE id = ? AND revoked = 0")
                .bind(id)
                .execute(&self.pool)
                .await?;
        Ok(done.rows_affected() == 1)
    }

    async fn revoke_refresh_token(&self, fingerprint: &str) -> AppResult<()> {
        sqlx::query("UPDATE refresh_tokens SET revoked = 1 WHERE token_hash = ?")
            .bind(fingerprint)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn revoke_user_refresh_tokens(&self, user_id: &str) -> AppResult<()> {
        sqlx::query("UPDATE refresh_tokens SET revoked = 1 WHERE user_id = ? AND revoked = 0")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_refresh_tokens_expired_before(&self, instant: &str) -> AppResult<u64> {
        let done = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < ?")
            .bind(instant)
            .execute(&self.pool)
            .await?;
        Ok(done.rows_affected())
    }
}
