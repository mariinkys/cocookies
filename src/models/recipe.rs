use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{recipe_ingredient::RecipeIngredient, recipe_step::RecipeStep};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Recipe {
    pub recipe_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub main_photo: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// This struct is used to display all the recipe info, it's not in the database like this
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct FullRecipe {
    pub recipe_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub main_photo: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
}
