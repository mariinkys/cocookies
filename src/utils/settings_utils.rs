use chrono::NaiveDateTime;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    recipe_ingredient::RecipeIngredient, recipe_note::RecipeNote, recipe_step::RecipeStep,
};

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

    #[serde(default)]
    pub ingredients: Vec<RecipeIngredient>,
    #[serde(default)]
    pub steps: Vec<RecipeStep>,
    #[serde(default)]
    pub notes: Vec<RecipeNote>,
}

#[server(GetFullRecipe, "/api/recipe/full")]
pub async fn get_full_recipes() -> Result<Vec<FullRecipe>, ServerFnError> {
    use crate::models::recipe_ingredient::RecipeIngredient;
    use crate::models::recipe_note::RecipeNote;
    use crate::models::recipe_step::RecipeStep;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sqlx::{Pool, Row, Sqlite};
    use std::collections::HashMap;
    use std::sync::Arc;

    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let query = r#"
        SELECT
            r.recipe_id, r.name, r.description, r.prep_time_minutes, r.servings,
            r.main_photo, r.created_at, r.updated_at,
            ri.recipe_ingredient_id, ri.ingredient_name, ri.quantity, ri.unit,
            rs.step_id, rs.step_number, rs.instructions,
            rn.note_id, rn.note
        FROM
            recipes r
        LEFT JOIN
            recipe_ingredients ri ON r.recipe_id = ri.recipe_id
        LEFT JOIN
            recipe_steps rs ON r.recipe_id = rs.recipe_id
        LEFT JOIN
            recipe_notes rn ON r.recipe_id = rn.recipe_id
        ORDER BY
            r.recipe_id, rs.step_number ASC
    "#;

    let rows = sqlx::query(query).fetch_all(&*pool).await;
    match rows {
        Ok(results) => {
            if results.is_empty() {
                return Ok(Vec::new()); 
            }

            let mut recipes_map: HashMap<i32, FullRecipe> = HashMap::new();

            for row in results {

                let recipe_id: i32 = row.try_get("recipe_id").unwrap_or_default();

                let recipe = recipes_map.entry(recipe_id).or_insert_with(|| FullRecipe {
                    recipe_id: Some(recipe_id),
                    name: row.try_get("name").unwrap_or_default(),
                    description: row.try_get("description").ok(),
                    prep_time_minutes: row.try_get("prep_time_minutes").ok(),
                    servings: row.try_get("servings").ok(),
                    main_photo: row.try_get("main_photo").ok(),
                    created_at: row.try_get("created_at").ok(),
                    updated_at: row.try_get("updated_at").ok(),
                    ingredients: Vec::new(),
                    steps: Vec::new(),
                    notes: Vec::new(),
                });

                let mut ingredient_map: HashMap<i32, RecipeIngredient> = HashMap::new();
                let mut step_map: HashMap<i32, RecipeStep> = HashMap::new();
                let mut note_map: HashMap<i32, RecipeNote> = HashMap::new();

                if let Ok(ingredient_id) = row.try_get::<i32, _>("recipe_ingredient_id") {
                    if ingredient_id > 0 {
                        ingredient_map
                            .entry(ingredient_id)
                            .or_insert_with(|| RecipeIngredient {
                                recipe_ingredient_id: Some(ingredient_id),
                                recipe_id,
                                ingredient_name: row.try_get("ingredient_name").unwrap_or_default(),
                                quantity: row.try_get("quantity").unwrap_or_default(),
                                unit: row.try_get("unit").ok(),
                                created_at: row.try_get("created_at").ok(),
                                updated_at: row.try_get("updated_at").ok(),
                            });
                    }
                }

                if let Ok(step_id) = row.try_get::<i32, _>("step_id") {
                    if step_id > 0 {
                        step_map.entry(step_id).or_insert_with(|| RecipeStep {
                            step_id: Some(step_id),
                            recipe_id,
                            step_number: row.try_get("step_number").unwrap_or_default(),
                            instructions: row.try_get("instructions").unwrap_or_default(),
                            created_at: row.try_get("created_at").ok(),
                            updated_at: row.try_get("updated_at").ok(),
                        });
                    }
                }

                if let Ok(note_id) = row.try_get::<i32, _>("note_id") {
                    if note_id > 0 {
                        note_map.entry(note_id).or_insert_with(|| RecipeNote {
                            note_id: Some(note_id),
                            recipe_id,
                            note: row.try_get("note").unwrap_or_default(),
                            created_at: row.try_get("created_at").ok(),
                            updated_at: row.try_get("updated_at").ok(),
                        });
                    }
                }

                recipe.ingredients = ingredient_map.into_values().collect();
                recipe.steps = step_map.into_values().collect();
                recipe.notes = note_map.into_values().collect();
            }

            let mut recipes: Vec<FullRecipe> = recipes_map.into_values().collect();

            for recipe in &mut recipes {
                recipe
                    .ingredients
                    .sort_by_key(|ingredient| ingredient.recipe_ingredient_id);
                recipe.steps.sort_by_key(|step| step.step_number);
                recipe.notes.sort_by_key(|note| note.note_id);
            }
            Ok(recipes)
        }
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}

#[server(InsertFullRecipe, "/api/recipe/full/insert")]
pub async fn insert_full_recipes(recipes: Vec<FullRecipe>) -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sqlx::{Pool, Sqlite};
    use std::sync::Arc;

    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    for recipe in recipes {
        let row = sqlx::query(
            r#"
            INSERT INTO recipes (name, description, prep_time_minutes, servings, main_photo, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(recipe.name)
        .bind(recipe.description)
        .bind(recipe.prep_time_minutes)
        .bind(recipe.servings)
        .bind(recipe.main_photo)
        .bind(recipe.created_at)
        .bind(recipe.updated_at)
        .execute(&*pool)  
        .await?;

        let recipe_id = row.last_insert_rowid();

        for ingredient in recipe.ingredients {
            sqlx::query(
                r#"
                INSERT INTO recipe_ingredients (recipe_id, ingredient_name, quantity, unit, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(recipe_id)
            .bind(ingredient.ingredient_name)
            .bind(ingredient.quantity)
            .bind(ingredient.unit)
            .bind(ingredient.created_at)
            .bind(ingredient.updated_at)
            .execute(&*pool)  // Dereference the Arc here
            .await?;
        }

        // Insert the steps associated with the recipe
        for step in recipe.steps {
            sqlx::query(
                r#"
                INSERT INTO recipe_steps (recipe_id, step_number, instructions, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?)
                "#,
            )
            .bind(recipe_id)
            .bind(step.step_number)
            .bind(step.instructions)
            .bind(step.created_at)
            .bind(step.updated_at)
            .execute(&*pool)  
            .await?;
        }

        for note in recipe.notes {
            sqlx::query(
                r#"
                INSERT INTO recipe_notes (recipe_id, note, created_at, updated_at)
                VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(recipe_id)
            .bind(note.note)
            .bind(note.created_at)
            .bind(note.updated_at)
            .execute(&*pool) 
            .await?;
        }
    }

    Ok(())
}
