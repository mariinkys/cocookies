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

use crate::models::recipe::Recipe;

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
pub async fn delete_recipe(id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if id != 0 {
        let row_result = sqlx::query("DELETE FROM recipes WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await;

        match row_result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::RowNotFound) => {
                Err(ServerFnError::new(String::from("Recipe not found")))
            }
            Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
        }
    } else {
        Err(ServerFnError::new(String::from("Query Error")))
    }
}
