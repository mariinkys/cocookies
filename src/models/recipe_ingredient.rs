use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RecipeIngredient {
    pub recipe_ingredient_id: Option<i32>,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub quantity: Option<String>,
    pub unit: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl RecipeIngredient {
    pub fn init(recipe_id: i32) -> Self {
        RecipeIngredient {
            recipe_ingredient_id: None,
            recipe_id,
            ingredient_name: String::new(),
            quantity: None,
            unit: None,
            created_at: None,
            updated_at: None,
        }
    }
}
