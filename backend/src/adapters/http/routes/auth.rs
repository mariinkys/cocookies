//! | method | path         | notes                                       |
//! |--------|--------------|---------------------------------------------|
//! | POST   | `/register`  | rate limited per IP                         |
//! | POST   | `/login`     | rate limited per IP, sets session cookies   |
//! | POST   | `/refresh`   | rotates the refresh token cookie            |
//! | POST   | `/logout`    | revokes sessions, clears cookies            |

use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::{Deserialize, Serialize};

use super::users::UserResponse;
use crate::{
    adapters::http::{
        app_state::AppState,
        cookies,
        extractors::{Json, OptionalUser},
        rate_limit,
    },
    app_error::AppResult,
    entities::{
        session::Session,
        user::{Credentials, NewUser, User},
    },
    infra::config::AppConfig,
    use_cases::{auth::AuthUseCases, user::UserUseCases},
};

pub fn router(state: AppState) -> Router<AppState> {
    let credentials = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route_layer(middleware::from_fn_with_state(
            state,
            rate_limit::limit_credentials,
        ));

    Router::new()
        .merge(credentials)
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
}

/// Body returned by login/refresh: just enough for the client to know who it is.
#[derive(Debug, Serialize)]
struct AuthResponse {
    id: String,
    email: String,
}

impl From<User> for AuthResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
        }
    }
}

async fn register(
    State(users): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    let user = users
        .register(NewUser {
            name: payload.name,
            email: payload.email,
            password: payload.password,
        })
        .await?;
    Ok((StatusCode::CREATED, axum::Json(UserResponse::from(user))))
}

async fn login(
    State(auth): State<Arc<AuthUseCases>>,
    State(config): State<Arc<AppConfig>>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<Response> {
    let (user, session) = auth
        .login(Credentials {
            email: payload.email,
            password: payload.password,
        })
        .await?;
    Ok(with_session(&config, session, user))
}

async fn refresh(
    State(auth): State<Arc<AuthUseCases>>,
    State(config): State<Arc<AppConfig>>,
    headers: HeaderMap,
) -> Response {
    let candidates: Vec<&str> = cookies::read_all(&headers, cookies::REFRESH_TOKEN).collect();
    match auth.refresh(&candidates).await {
        Ok((user, session)) => with_session(&config, session, user),
        Err(err) => {
            // Make the browser forget a token that will never work again.
            let mut response = err.into_response();
            cookies::clear(response.headers_mut(), config.auth.cookie_secure);
            response
        }
    }
}

async fn logout(
    State(auth): State<Arc<AuthUseCases>>,
    State(config): State<Arc<AppConfig>>,
    OptionalUser(user): OptionalUser,
    headers: HeaderMap,
) -> AppResult<Response> {
    let tokens: Vec<&str> = cookies::read_all(&headers, cookies::REFRESH_TOKEN).collect();
    auth.logout(user.as_ref().map(|u| u.user_id.as_str()), &tokens)
        .await?;
    let mut response = StatusCode::NO_CONTENT.into_response();
    cookies::clear(response.headers_mut(), config.auth.cookie_secure);
    Ok(response)
}

fn with_session(config: &AppConfig, session: Session, user: User) -> Response {
    let mut response = axum::Json(AuthResponse::from(user)).into_response();
    cookies::session(
        response.headers_mut(),
        &session.access_token,
        config.auth.access_token_ttl.as_secs(),
        &session.refresh_token,
        config.auth.refresh_token_ttl.as_secs(),
        config.auth.cookie_secure,
    );
    response
}
