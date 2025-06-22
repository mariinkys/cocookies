// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RecipeNote {
    pub note_id: Option<i32>,
    pub recipe_id: i32,
    pub note: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl RecipeNote {
    pub fn init(recipe_id: i32) -> Self {
        RecipeNote {
            note_id: None,
            recipe_id,
            note: String::new(),
            created_at: None,
            updated_at: None,
        }
    }
}
