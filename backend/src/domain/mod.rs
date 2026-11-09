//! Domain layer
//!
//! Plain Rust only — no web framework, no database, no serialization. Everything else in the
//! crate depends on this module; it depends on nothing.

pub mod entities;
pub mod time;
pub mod validation;
