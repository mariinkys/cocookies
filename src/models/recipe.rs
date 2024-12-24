use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
