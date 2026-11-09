use async_trait::async_trait;
use sqlx::FromRow;

use super::{SqlitePersistence, new_id};
use crate::{
    app_error::{AppError, AppResult},
    entities::user::User,
    time::now_iso,
    use_cases::user::UserPersistence,
};

/// User as stored in the database.
#[derive(FromRow)]
struct UserDb {
    id: String,
    name: String,
    email: String,
    password: String,
    failed_login_attempts: i64,
    locked_until: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<UserDb> for User {
    fn from(db: UserDb) -> Self {
        User {
            id: db.id,
            name: db.name,
            email: db.email,
            password_hash: db.password,
            failed_login_attempts: db.failed_login_attempts,
            locked_until: db.locked_until,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

const SELECT_USER: &str = "SELECT id, name, email, password, failed_login_attempts, locked_until, \
                           created_at, updated_at FROM users";

#[async_trait]
impl UserPersistence for SqlitePersistence {
    async fn find_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let row: Option<UserDb> =
            sqlx::query_as(sqlx::AssertSqlSafe(format!("{SELECT_USER} WHERE id = ?")))
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(row.map(User::from))
    }

    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row: Option<UserDb> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
            "{SELECT_USER} WHERE email = ?"
        )))
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(User::from))
    }

    async fn user_email_taken(&self, email: &str, except_id: Option<&str>) -> AppResult<bool> {
        Ok(
            sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = ? AND id IS NOT ?)",
            )
            .bind(email)
            .bind(except_id)
            .fetch_one(&self.pool)
            .await?,
        )
    }

    async fn create_user(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User> {
        let id = new_id();
        let now = now_iso();
        sqlx::query(
            "INSERT INTO users (id, name, email, password, failed_login_attempts, created_at, updated_at)
             VALUES (?, ?, ?, ?, 0, ?, ?)",
        )
        .bind(&id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.find_user_by_id(&id)
            .await?
            .ok_or_else(|| AppError::internal("user vanished after insert"))
    }

    async fn update_user_profile(&self, id: &str, name: &str, email: &str) -> AppResult<()> {
        sqlx::query("UPDATE users SET name = ?, email = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(email)
            .bind(now_iso())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_user_password(&self, id: &str, password_hash: &str) -> AppResult<()> {
        sqlx::query("UPDATE users SET password = ?, updated_at = ? WHERE id = ?")
            .bind(password_hash)
            .bind(now_iso())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn record_failed_login(
        &self,
        id: &str,
        attempts: i64,
        locked_until: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query("UPDATE users SET failed_login_attempts = ?, locked_until = ? WHERE id = ?")
            .bind(attempts)
            .bind(locked_until)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn reset_failed_logins(&self, id: &str) -> AppResult<()> {
        sqlx::query("UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Categories, difficulties, recipes and refresh tokens cascade.
    async fn delete_user(&self, id: &str) -> AppResult<bool> {
        let done = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(done.rows_affected() == 1)
    }
}
