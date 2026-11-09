use async_trait::async_trait;

use super::SqlitePersistence;
use crate::{app_error::AppResult, use_cases::system::StorageMaintenance};

#[async_trait]
impl StorageMaintenance for SqlitePersistence {
    async fn ping(&self) -> AppResult<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    /// Lets SQLite refresh query-planner statistics when it considers them stale.
    async fn optimize(&self) -> AppResult<()> {
        sqlx::query("PRAGMA optimize").execute(&self.pool).await?;
        Ok(())
    }
}
