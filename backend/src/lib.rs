//! Cocookies backend, organised following Clean Architecture:
//!
//! ```text
//!   infra  ──►  adapters  ──►  application  ──►  domain
//! (wiring)   (http, sqlite,    (use cases +     (entities,
//!             crypto, pdf…)     their ports)      rules)
//! ```
//!
//! For more info I used this as reference: https://github.com/Thodin/axum-clean-architecture/

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infra;

// Re-exports
pub use application::*;
pub use domain::*;
