//! Route table of the API (mounted under `/api`).
//!
//! | prefix              | module        |
//! |---------------------|---------------|
//! | `/auth`             | [`auth`]      |
//! | `/users`            | [`users`]     |
//! | `/categories`       | [`taxonomy`]  |
//! | `/difficulties`     | [`taxonomy`]  |
//! | `/recipes`          | [`recipes`]   |
//! | `/llm`              | [`imports`]   |
//! | `/system`           | [`system`]    |

pub mod auth;
pub mod imports;
pub mod recipes;
pub mod system;
pub mod taxonomy;
pub mod users;

use axum::Router;

use crate::{adapters::http::app_state::AppState, entities::taxonomy::TaxonomyKind};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router(state))
        .nest("/users", users::router())
        .nest("/categories", taxonomy::router(TaxonomyKind::Category))
        .nest("/difficulties", taxonomy::router(TaxonomyKind::Difficulty))
        .nest("/recipes", recipes::router())
        .nest("/llm", imports::router())
        .nest("/system", system::router())
}
