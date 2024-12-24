use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RecipeIngredient {
    pub recipe_ingredient_id: Option<i32>,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub quantity: f64,
    pub unit: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
