//! Public information about the running server.

use std::sync::Arc;

use async_trait::async_trait;

use crate::app_error::AppResult;

/// Health and upkeep of the storage backend.
#[async_trait]
pub trait StorageMaintenance: Send + Sync {
    /// Checks that the storage answers.
    async fn ping(&self) -> AppResult<()>;
    /// Periodic, cheap housekeeping (e.g. refreshing query-planner statistics).
    async fn optimize(&self) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub version: &'static str,
    pub guest_access: bool,
}

#[derive(Clone)]
pub struct SystemUseCases {
    storage: Arc<dyn StorageMaintenance>,
    info: ServerInfo,
}

impl SystemUseCases {
    pub fn new(storage: Arc<dyn StorageMaintenance>, info: ServerInfo) -> Self {
        Self { storage, info }
    }

    pub fn info(&self) -> &ServerInfo {
        &self.info
    }

    pub async fn health(&self) -> AppResult<()> {
        self.storage.ping().await
    }

    pub async fn optimize_storage(&self) -> AppResult<()> {
        self.storage.optimize().await
    }
}
