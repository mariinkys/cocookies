//! Periodic housekeeping, run in the background every hour.

use std::time::Duration;

use tracing::{info, warn};

use crate::adapters::http::app_state::AppState;

const INTERVAL: Duration = Duration::from_secs(3600);

pub fn spawn(state: AppState) {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(INTERVAL);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            run_once(&state).await;
        }
    });
}

pub async fn run_once(state: &AppState) {
    match state.auth_use_cases.purge_expired_sessions().await {
        Ok(0) => {}
        Ok(n) => info!(deleted = n, "removed expired refresh tokens"),
        Err(e) => warn!(error = %e, "refresh-token cleanup failed"),
    }

    state.rate_limiter.prune();

    if let Err(e) = state.system_use_cases.optimize_storage().await {
        warn!(error = %e, "storage maintenance failed");
    }
}
