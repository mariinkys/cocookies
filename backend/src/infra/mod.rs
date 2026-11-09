//! Infrastructure: configuration, database bootstrap, router composition and the wiring of
//! concrete adapters into use cases (the only place that knows about every layer).

pub mod app;
pub mod config;
pub mod db;
pub mod jobs;
pub mod setup;
