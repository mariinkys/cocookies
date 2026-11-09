//! | method | path                   | notes                    |
//! |--------|------------------------|--------------------------|
//! | GET    | `/me`                  | the signed-in user       |
//! | GET    | `/{id}`                | self only                |
//! | PUT    | `/{id}`                | self only (name, email)  |
//! | PATCH  | `/{id}/password`       | self only                |
//! | DELETE | `/{id}`                | self only, cascades      |

use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        cookies,
        extractors::{AuthUser, Json, Path},
    },
    app_error::AppResult,
    entities::user::{PasswordChange, ProfileUpdate, User},
    infra::config::AppConfig,
    use_cases::user::UserUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(me))
        .route("/{id}", get(get_user).put(update).delete(delete))
        .route("/{id}/password", patch(change_password))
}

/// Public representation of a user (never includes the password hash).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    id: String,
    name: String,
    email: String,
    created_at: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdatePayload {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordPayload {
    #[serde(default)]
    current_password: Option<String>,
    #[serde(default)]
    new_password: String,
}

async fn me(
    State(users): State<Arc<UserUseCases>>,
    AuthUser(me): AuthUser,
) -> AppResult<axum::Json<UserResponse>> {
    let id = me.user_id.clone();
    Ok(axum::Json(users.get(&me, &id).await?.into()))
}

async fn get_user(
    State(users): State<Arc<UserUseCases>>,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<axum::Json<UserResponse>> {
    Ok(axum::Json(users.get(&me, &id.to_string()).await?.into()))
}

async fn update(
    State(users): State<Arc<UserUseCases>>,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePayload>,
) -> AppResult<axum::Json<UserResponse>> {
    let update = ProfileUpdate {
        name: payload.name,
        email: payload.email,
    };
    Ok(axum::Json(
        users.update(&me, &id.to_string(), update).await?.into(),
    ))
}

async fn change_password(
    State(users): State<Arc<UserUseCases>>,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ChangePasswordPayload>,
) -> AppResult<StatusCode> {
    let change = PasswordChange {
        current_password: payload.current_password,
        new_password: payload.new_password,
    };
    users.change_password(&me, &id.to_string(), change).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete(
    State(users): State<Arc<UserUseCases>>,
    State(config): State<Arc<AppConfig>>,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Response> {
    users.delete(&me, &id.to_string()).await?;
    let mut response = StatusCode::NO_CONTENT.into_response();
    cookies::clear(response.headers_mut(), config.auth.cookie_secure);
    Ok(response)
}
