//! The REST API: axum handlers that translate HTTP into use-case calls and back.
//!
//! Each file in [`routes`] owns the request/response DTOs (the JSON contract with the
//! frontend) of one feature and converts them from/to domain entities, so the domain never
//! depends on serde.

pub mod app_error_impl;
pub mod app_state;
pub mod cookies;
pub mod extractors;
pub mod pagination;
pub mod rate_limit;
pub mod routes;
