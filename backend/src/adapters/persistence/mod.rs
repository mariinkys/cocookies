//! SQLite persistence. One [`SqlitePersistence`] value implements every persistence port; each
//! sub-module holds the SQL and row types of one aggregate.
//!
//! Dynamic SQL is only ever assembled from compile-time constants (table names, whitelisted
//! sort expressions); user input is always bound as a parameter.

mod health;
mod recipe;
mod session;
mod taxonomy;
mod user;

use sqlx::SqlitePool;

use crate::app_error::AppError;

#[derive(Clone)]
pub struct SqlitePersistence {
    pool: SqlitePool,
}

impl SqlitePersistence {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        match &value {
            sqlx::Error::Database(db) if db.is_unique_violation() => AppError::UniqueViolation,
            _ => AppError::Database(value.to_string()),
        }
    }
}

/// A fresh random identifier in the canonical hyphenated form used by the schema.
fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// `LIKE` pattern matching `term` anywhere, escaping `%`, `_` and `\` (use with `ESCAPE '\'`).
fn contains_pattern(term: &str) -> String {
    let mut out = String::with_capacity(term.len() + 2);
    out.push('%');
    for c in term.chars() {
        if matches!(c, '%' | '_' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('%');
    out
}

fn direction_sql(d: crate::entities::page::SortDirection) -> &'static str {
    match d {
        crate::entities::page::SortDirection::Asc => "ASC",
        crate::entities::page::SortDirection::Desc => "DESC",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn like_pattern_is_escaped() {
        assert_eq!(super::contains_pattern("50%_off\\"), "%50\\%\\_off\\\\%");
    }
}
