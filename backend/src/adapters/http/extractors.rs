//! Request extractors.
//!
//! * [`AuthUser`] / [`OptionalUser`] turn the `access_token` cookie into an [`Identity`] via
//!   the auth use cases.
//! * [`Json`], [`Query`] and [`Path`] wrap axum's extractors so malformed input is reported
//!   with the standard JSON error body instead of plain text.

use std::sync::Arc;

use axum::{
    extract::{
        FromRef, FromRequest, FromRequestParts, Request,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use super::cookies;
use crate::{
    app_error::{AppError, AppResult},
    entities::session::Identity,
    use_cases::auth::AuthUseCases,
};

/// A signed-in user (otherwise `401`).
#[derive(Debug, Clone)]
pub struct AuthUser(pub Identity);

/// The signed-in user, or `None` for an anonymous visitor (never rejects).
#[derive(Debug, Clone)]
pub struct OptionalUser(pub Option<Identity>);

async fn resolve<S>(parts: &Parts, state: &S) -> AppResult<Option<Identity>>
where
    Arc<AuthUseCases>: FromRef<S>,
{
    if let Some(identity) = parts.extensions.get::<Identity>() {
        return Ok(Some(identity.clone()));
    }
    match cookies::read(&parts.headers, cookies::ACCESS_TOKEN) {
        Some(token) => {
            Arc::<AuthUseCases>::from_ref(state)
                .authenticate(token)
                .await
        }
        None => Ok(None),
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AuthUseCases>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let identity = resolve(parts, state)
            .await?
            .ok_or_else(AppError::unauthenticated)?;
        parts.extensions.insert(identity.clone());
        Ok(Self(identity))
    }
}

impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    Arc<AuthUseCases>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(resolve(parts, state).await?))
    }
}

/// `axum::Json` with JSON error responses.
pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state).await?;
        Ok(Self(value))
    }
}

/// `axum::extract::Query` with JSON error responses.
pub struct Query<T>(pub T);

impl<S, T> FromRequestParts<S> for Query<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(v) =
            axum::extract::Query::<T>::from_request_parts(parts, state).await?;
        Ok(Self(v))
    }
}

/// `axum::extract::Path` with JSON error responses.
pub struct Path<T>(pub T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(v) =
            axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        Ok(Self(v))
    }
}

impl From<JsonRejection> for AppError {
    fn from(r: JsonRejection) -> Self {
        Self::Validation(r.body_text())
    }
}

impl From<QueryRejection> for AppError {
    fn from(r: QueryRejection) -> Self {
        Self::Validation(r.body_text())
    }
}

impl From<PathRejection> for AppError {
    fn from(r: PathRejection) -> Self {
        Self::Validation(r.body_text())
    }
}
