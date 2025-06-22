// SPDX-License-Identifier: GPL-3.0-only

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

use crate::models::recipe_ingredient::RecipeIngredient;

#[server(GetAllRecipeIngredients, "/api/recipe_ingredients")]
pub async fn get_all_recipe_ingredients(
    recipe_id: i32,
) -> Result<Vec<RecipeIngredient>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let mut rows =
        sqlx::query("SELECT * FROM recipe_ingredients WHERE recipe_id = ? ORDER BY recipe_id ASC")
            .bind(recipe_id)
            .fetch(&*pool);

    let mut result = Vec::<RecipeIngredient>::new();

    while let Some(row) = rows.try_next().await? {
        let recipe_ingredient_id: Option<i32> = row.try_get("recipe_ingredient_id").unwrap_or(None);
        let recipe_id: i32 = row.try_get("recipe_id").unwrap_or_default();
        let ingredient_name = row.try_get("ingredient_name").unwrap_or("Error");
        let quantity = row.try_get("quantity").unwrap_or(None);
        let unit = row.try_get("unit").unwrap_or(None);
        let created_at: Option<NaiveDateTime> = row.try_get("created_at").unwrap_or(None);
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at").unwrap_or(None);

        let recipe_ingredient = RecipeIngredient {
            recipe_ingredient_id,
            recipe_id,
            ingredient_name: ingredient_name.to_string(),
            quantity,
            unit,
            created_at,
            updated_at,
        };

        if let Some(_id) = recipe_ingredient.recipe_ingredient_id {
            result.push(recipe_ingredient);
        }
    }

    Ok(result)
}

#[server(AddRecipeIngredients, "/api/recipeingredients/add")]
pub async fn add_recipe_ingredients(
    recipe_id: i32,
    ingredient_name: String,
    quantity: Option<String>,
    unit: Option<String>,
) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "INSERT INTO recipe_ingredients (
                recipe_id,
                ingredient_name,
                quantity,
                unit
            )
            VALUES (?, ?, ?, ?)
        ",
    )
    .bind(recipe_id)
    .bind(ingredient_name)
    .bind(quantity)
    .bind(unit)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(UpdateRecipeIngredients, "/api/recipeingredients/update")]
pub async fn update_recipe_ingredients(
    recipe_ingredient_id: i32,
    ingredient_name: String,
    quantity: Option<String>,
    unit: Option<String>,
) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "UPDATE recipe_ingredients
            SET
                ingredient_name = ?,
                quantity = ?,
                unit = ?
            WHERE
                recipe_ingredient_id = ?
        ",
    )
    .bind(ingredient_name)
    .bind(quantity)
    .bind(unit)
    .bind(recipe_ingredient_id)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(DeleteRecipeIngredient, "/api/recipeingredients/delete")]
pub async fn delete_recipeingredients(recipe_ingredient_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if recipe_ingredient_id == 0 {
        return Err(ServerFnError::ServerError(String::from(
            "Invalid recipe_ingredient_id",
        )));
    };

    let delete_row_result =
        sqlx::query("DELETE FROM recipe_ingredients WHERE recipe_ingredient_id = ?")
            .bind(recipe_ingredient_id)
            .execute(&*pool)
            .await;

    match delete_row_result {
        Ok(_) => Ok(()),
        Err(Error::RowNotFound) => Err(ServerFnError::ServerError(String::from(
            "Recipe ingredient not found",
        ))),
        Err(err) => Err(ServerFnError::ServerError(format!(
            "Failed to delete recipe ingredient: {err}"
        ))),
    }
}
