//! | method | path        | response                                   |
//! |--------|-------------|--------------------------------------------|
//! | GET    | `/version`  | plain-text version, e.g. `0.3.0`           |
//! | GET    | `/config`   | `{"version": "...", "guestAccess": true}`  |
//! | GET    | `/health`   | `200 ok` when the database answers         |

use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::{
    adapters::http::app_state::AppState, app_error::AppResult, use_cases::system::SystemUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/version", get(version))
        .route("/config", get(config))
        .route("/health", get(health))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicConfig {
    version: &'static str,
    guest_access: bool,
}

async fn version(State(system): State<Arc<SystemUseCases>>) -> &'static str {
    system.info().version
}

async fn config(State(system): State<Arc<SystemUseCases>>) -> Json<PublicConfig> {
    let info = system.info();
    Json(PublicConfig {
        version: info.version,
        guest_access: info.guest_access,
    })
}

async fn health(State(system): State<Arc<SystemUseCases>>) -> AppResult<&'static str> {
    system.health().await?;
    Ok("ok")
}
