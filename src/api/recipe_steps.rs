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

use crate::models::recipe_step::RecipeStep;

#[server(GetAllRecipeSteps, "/api/recipe_steps")]
pub async fn get_all_recipe_steps(recipe_id: i32) -> Result<Vec<RecipeStep>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let mut rows =
        sqlx::query("SELECT * FROM recipe_steps WHERE recipe_id = ? ORDER BY step_number ASC")
            .bind(recipe_id)
            .fetch(&*pool);

    let mut result = Vec::<RecipeStep>::new();

    while let Some(row) = rows.try_next().await? {
        let step_id: Option<i32> = row.try_get("step_id").unwrap_or(None);
        let recipe_id: i32 = row.try_get("recipe_id").unwrap_or_default();
        let step_number: i32 = row.try_get("step_number").unwrap_or_default();
        let instructions = row.try_get("instructions").unwrap_or("Error");
        let created_at: Option<NaiveDateTime> = row.try_get("created_at").unwrap_or(None);
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at").unwrap_or(None);

        let recipe_step = RecipeStep {
            step_id,
            recipe_id,
            step_number,
            instructions: instructions.to_string(),
            created_at,
            updated_at,
        };

        if let Some(_id) = recipe_step.step_id {
            result.push(recipe_step);
        }
    }

    Ok(result)
}

#[server(AddRecipeSteps, "/api/recipesteps/add")]
pub async fn add_recipe_steps(
    recipe_id: i32,
    step_number: i32,
    instructions: String,
) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "INSERT INTO recipe_steps (
                recipe_id,
                step_number,
                instructions
            )
            VALUES (?, ?, ?)
        ",
    )
    .bind(recipe_id)
    .bind(step_number)
    .bind(instructions)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(UpdateRecipeSteps, "/api/recipesteps/update")]
pub async fn update_recipe_steps(
    step_id: i32,
    step_number: i32,
    instructions: String,
) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "UPDATE recipe_steps
            SET
                step_number = ?,
                instructions = ?
            WHERE
                step_id = ?
        ",
    )
    .bind(step_number)
    .bind(instructions)
    .bind(step_id)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(DeleteRecipeStep, "/api/recipesteps/delete")]
pub async fn delete_steps(step_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if step_id == 0 {
        return Err(ServerFnError::ServerError(String::from("Invalid step_id")));
    };

    let delete_row_result = sqlx::query("DELETE FROM recipe_steps WHERE step_id = ?")
        .bind(step_id)
        .execute(&*pool)
        .await;

    match delete_row_result {
        Ok(_) => Ok(()),
        Err(Error::RowNotFound) => Err(ServerFnError::ServerError(String::from(
            "Recipe step not found",
        ))),
        Err(err) => Err(ServerFnError::ServerError(format!(
            "Failed to delete step: {err}"
        ))),
    }
}
