use leptos::prelude::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use futures::TryStreamExt;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Error, Pool, Row, Sqlite};
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::models::recipe::{FullRecipe, Recipe};

#[server(GetRecipe, "/api/recipe")]
pub async fn get_recipe(id: i32) -> Result<Recipe, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let row_result = sqlx::query("SELECT * FROM recipes WHERE recipe_id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await;

    match row_result {
        Ok(row) => {
            let recipe: Recipe = Recipe {
                recipe_id: row.get("recipe_id"),
                name: row.get("name"),
                description: row.get("description"),
                prep_time_minutes: row.get("prep_time_minutes"),
                servings: row.get("servings"),
                main_photo: row.get("main_photo"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(recipe)
        }
        Err(Error::RowNotFound) => Err(ServerFnError::new(String::from("Recipe not found"))),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}

#[server(AddRecipe, "/api/recipe/add")]
pub async fn add_recipe(recipe: Recipe) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "INSERT INTO recipes (
                name,
                description,
                prep_time_minutes,
                servings,
                main_photo
            )
            VALUES (?, ?, ?, ?, ?)
        ",
    )
    .bind(recipe.name)
    .bind(recipe.description)
    .bind(recipe.prep_time_minutes)
    .bind(recipe.servings)
    .bind(recipe.main_photo)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}

#[server(UpdateRecipe, "/api/recipe/update")]
pub async fn update_recipe(
    recipe_id: Option<i32>,
    name: String,
    description: Option<String>,
    prep_time_minutes: Option<i32>,
    servings: Option<i32>,
) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "UPDATE recipes
            SET
                name = ?,
                description = ?,
                prep_time_minutes = ?,
                servings = ?
            WHERE
                recipe_id = ?
        ",
    )
    .bind(name)
    .bind(description)
    .bind(prep_time_minutes)
    .bind(servings)
    .bind(recipe_id)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}

#[server(GetAllRecipes, "/api/recipes")]
pub async fn get_all_recipes() -> Result<Vec<Recipe>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let mut rows = sqlx::query("SELECT * FROM recipes ORDER BY recipe_id ASC").fetch(&*pool);

    let mut result = Vec::<Recipe>::new();

    while let Some(row) = rows.try_next().await? {
        let recipe_id: Option<i32> = row.try_get("recipe_id").unwrap_or(None);
        let name = row.try_get("name").unwrap_or("Error");
        let description: Option<String> = row.try_get("description").unwrap_or(None);
        let prep_time_minutes: Option<i32> = row.try_get("prep_time_minutes").unwrap_or(None);
        let servings: Option<i32> = row.try_get("servings").unwrap_or(None);
        let main_photo: Option<String> = row.try_get("main_photo").unwrap_or(None);
        let created_at: Option<NaiveDateTime> = row.try_get("created_at").unwrap_or(None);
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at").unwrap_or(None);

        let recipe = Recipe {
            recipe_id,
            name: String::from(name),
            description,
            prep_time_minutes,
            servings,
            main_photo,
            created_at,
            updated_at,
        };

        if let Some(_id) = recipe.recipe_id {
            result.push(recipe);
        }
    }

    // TODO: Errors are not handled correctly on the view, it just says 'No Recipes', how to trigger the ErrorBoundary?
    //return Err(ServerFnError::new(String::from("Testing Error")));

    Ok(result)
}

#[server(DeleteRecipe, "/api/recipe/delete")]
pub async fn delete_recipe(recipe_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if recipe_id == 0 {
        return Err(ServerFnError::ServerError(String::from(
            "Invalid recipe_id",
        )));
    }

    let row_result = sqlx::query("SELECT * FROM recipes WHERE recipe_id = ?")
        .bind(recipe_id)
        .fetch_one(&*pool)
        .await;

    let recipe = match row_result {
        Ok(row) => Recipe {
            recipe_id: row.get("recipe_id"),
            name: row.get("name"),
            description: row.get("description"),
            prep_time_minutes: row.get("prep_time_minutes"),
            servings: row.get("servings"),
            main_photo: row.get("main_photo"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        Err(Error::RowNotFound) => {
            return Err(ServerFnError::ServerError(String::from("Recipe not found")));
        }
        Err(err) => {
            return Err(ServerFnError::ServerError(format!("Server Error: {}", err)));
        }
    };

    // Attempt to delete the main photo, if it exists
    if recipe.main_photo.is_some() {
        if let Err(err) = crate::utils::delete_file(recipe.main_photo.unwrap()).await {
            return Err(ServerFnError::ServerError(format!(
                "Failed to delete main photo: {}",
                err
            )));
        }
    }

    // Delete the recipe from the database
    let delete_row_result = sqlx::query("DELETE FROM recipes WHERE recipe_id = ?")
        .bind(recipe_id)
        .execute(&*pool)
        .await;

    match delete_row_result {
        Ok(_) => Ok(()),
        Err(Error::RowNotFound) => {
            Err(ServerFnError::ServerError(String::from("Recipe not found")))
        }
        Err(err) => Err(ServerFnError::ServerError(format!(
            "Failed to delete recipe: {}",
            err
        ))),
    }
}

#[server(GetFullRecipe, "/api/recipe/full")]
pub async fn get_full_recipe(id: i32) -> Result<FullRecipe, ServerFnError> {
    use crate::models::recipe_ingredient::RecipeIngredient;
    use crate::models::recipe_step::RecipeStep;
    use sqlx::{Error, Row};
    use std::collections::HashMap;

    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let query = r#"
        SELECT
            r.recipe_id, r.name, r.description, r.prep_time_minutes, r.servings,
            r.main_photo, r.created_at, r.updated_at,
            ri.recipe_ingredient_id, ri.ingredient_name, ri.quantity, ri.unit,
            rs.step_id, rs.step_number, rs.instructions
        FROM
            recipes r
        LEFT JOIN
            recipe_ingredients ri ON r.recipe_id = ri.recipe_id
        LEFT JOIN
            recipe_steps rs ON r.recipe_id = rs.recipe_id
        WHERE
            r.recipe_id = ?
        ORDER BY
            rs.step_number ASC
    "#;

    let rows = sqlx::query(query).bind(id).fetch_all(&*pool).await;

    match rows {
        Ok(results) => {
            if results.is_empty() {
                return Err(ServerFnError::new("Recipe not found".to_string()));
            }

            // Initialize containers for unique ingredients and steps (avoid duplicates)
            let mut ingredient_map: HashMap<i32, RecipeIngredient> = HashMap::new();
            let mut step_map: HashMap<i32, RecipeStep> = HashMap::new();

            let first_row = &results[0];
            let full_recipe = FullRecipe {
                recipe_id: first_row.try_get("recipe_id").ok(),
                name: first_row.try_get("name").unwrap_or_default(),
                description: first_row.try_get("description").ok(),
                prep_time_minutes: first_row.try_get("prep_time_minutes").ok(),
                servings: first_row.try_get("servings").ok(),
                main_photo: first_row.try_get("main_photo").ok(),
                created_at: first_row.try_get("created_at").ok(),
                updated_at: first_row.try_get("updated_at").ok(),
                ingredients: Vec::new(),
                steps: Vec::new(),
            };

            for row in results {
                if let Ok(ingredient_id) = row.try_get::<i32, _>("recipe_ingredient_id") {
                    ingredient_map
                        .entry(ingredient_id)
                        .or_insert_with(|| RecipeIngredient {
                            recipe_ingredient_id: Some(ingredient_id),
                            recipe_id: row.try_get("recipe_id").unwrap_or_default(),
                            ingredient_name: row.try_get("ingredient_name").unwrap_or_default(),
                            quantity: row.try_get("quantity").unwrap_or_default(),
                            unit: row.try_get("unit").ok(),
                            created_at: row.try_get("created_at").ok(),
                            updated_at: row.try_get("updated_at").ok(),
                        });
                }

                if let Ok(step_id) = row.try_get::<i32, _>("step_id") {
                    step_map.entry(step_id).or_insert_with(|| RecipeStep {
                        step_id: Some(step_id),
                        recipe_id: row.try_get("recipe_id").unwrap_or_default(),
                        step_number: row.try_get("step_number").unwrap_or_default(),
                        instructions: row.try_get("instructions").unwrap_or_default(),
                        created_at: row.try_get("created_at").ok(),
                        updated_at: row.try_get("updated_at").ok(),
                    });
                }
            }

            // Convert maps to vectors
            let mut ingredients: Vec<RecipeIngredient> = ingredient_map.into_values().collect();
            let mut steps: Vec<RecipeStep> = step_map.into_values().collect();

            ingredients.sort_by_key(|ingredient| ingredient.recipe_ingredient_id);
            steps.sort_by_key(|step| step.step_number);

            Ok(FullRecipe {
                ingredients,
                steps,
                ..full_recipe
            })
        }
        Err(Error::RowNotFound) => Err(ServerFnError::new("Recipe not found".to_string())),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}
