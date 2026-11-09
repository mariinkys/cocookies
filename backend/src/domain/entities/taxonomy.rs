//! Per-user taxonomies used to organise recipes: categories and difficulties.
//!
//! Both have exactly the same shape and rules, so they are one entity distinguished by
//! [`TaxonomyKind`]. Adding a third one (e.g. cuisines) is a new variant plus a table.

use crate::domain::validation::{ValidationError, Violations, max_chars, normalize_opt, not_blank};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxonomyKind {
    Category,
    Difficulty,
}

impl TaxonomyKind {
    /// Human readable name used in messages ("Category not found ...").
    pub fn label(self) -> &'static str {
        match self {
            Self::Category => "Category",
            Self::Difficulty => "Difficulty",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaxonomyItem {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Input to create or update a category / difficulty.
#[derive(Debug, Clone)]
pub struct TaxonomyDraft {
    pub name: String,
    pub description: Option<String>,
}

impl TaxonomyDraft {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        v.check(not_blank(&self.name), "Name is required")
            .check(
                max_chars(self.name.trim(), 100),
                "Name must not exceed 100 characters",
            )
            .check(
                self.description
                    .as_deref()
                    .is_none_or(|d| max_chars(d, 2000)),
                "Description must not exceed 2000 characters",
            );
        v.finish()
    }

    /// Trimmed name, empty description turned into `None`.
    pub fn normalized(self) -> Self {
        Self {
            name: self.name.trim().to_owned(),
            description: normalize_opt(self.description),
        }
    }
}

/// Fields taxonomy listings can be sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaxonomySort {
    #[default]
    Name,
    CreatedAt,
    UpdatedAt,
}
