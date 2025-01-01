use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RecipeStep {
    pub step_id: Option<i32>,
    pub recipe_id: i32,
    pub step_number: i32,
    pub instructions: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl RecipeStep {
    pub fn init(recipe_id: i32) -> Self {
        RecipeStep {
            step_id: None,
            recipe_id,
            step_number: 0,
            instructions: String::new(),
            created_at: None,
            updated_at: None,
        }
    }
}
