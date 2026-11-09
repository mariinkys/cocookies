//! SQLite connection pool and migrations.

use std::{path::Path, str::FromStr, time::Duration};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use tracing::info;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// Opens (creating if needed) the database and applies pending migrations.
pub async fn init_db(path: &Path, max_connections: u32) -> anyhow::Result<SqlitePool> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }

    let options = SqliteConnectOptions::from_str("sqlite://")?
        .filename(path)
        .create_if_missing(true)
        // WAL lets readers proceed while a write is in progress.
        .journal_mode(SqliteJournalMode::Wal)
        // Safe with WAL and much faster than FULL.
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5))
        // Keep each connection's page cache modest (~4 MiB) and temp data in memory.
        .pragma("cache_size", "-4096")
        .pragma("temp_store", "MEMORY");

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections.max(1))
        .min_connections(1)
        .idle_timeout(Duration::from_secs(600))
        .connect_with(options)
        .await?;

    MIGRATOR.run(&pool).await?;
    info!(path = %path.display(), "database ready");
    Ok(pool)
}
