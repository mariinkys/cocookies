//! Application layer: what the system does.
//!
//! Each module in [`use_cases`] holds one `XxxUseCases` struct with the business operations of a
//! feature, plus the **ports** (traits) it needs from the outside world — persistence, hashing,
//! PDF rendering, the LLM... Use cases only see those traits; the concrete implementations live
//! in `crate::adapters` and are wired together in `crate::infra::setup`. That keeps the rules
//! here testable with in-memory fakes and independent of axum, SQLite or krilla.

pub mod app_error;
pub mod use_cases;
