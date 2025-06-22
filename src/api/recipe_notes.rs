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

use crate::models::recipe_note::RecipeNote;

#[server(GetAllRecipeNotes, "/api/recipe_notes")]
pub async fn get_all_recipe_notes(recipe_id: i32) -> Result<Vec<RecipeNote>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let mut rows = sqlx::query("SELECT * FROM recipe_notes WHERE recipe_id = ?")
        .bind(recipe_id)
        .fetch(&*pool);

    let mut result = Vec::<RecipeNote>::new();

    while let Some(row) = rows.try_next().await? {
        let note_id: Option<i32> = row.try_get("note_id").unwrap_or(None);
        let recipe_id: i32 = row.try_get("recipe_id").unwrap_or_default();
        let note = row.try_get("note").unwrap_or_default();
        let created_at: Option<NaiveDateTime> = row.try_get("created_at").unwrap_or(None);
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at").unwrap_or(None);

        let recipe_note = RecipeNote {
            note_id,
            recipe_id,
            note,
            created_at,
            updated_at,
        };

        if let Some(_id) = recipe_note.note_id {
            result.push(recipe_note);
        }
    }

    Ok(result)
}

#[server(AddRecipeNotes, "/api/recipenotes/add")]
pub async fn add_recipe_notes(recipe_id: i32, note: String) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "INSERT INTO recipe_notes (
                recipe_id,
                note
            )
            VALUES (?, ?)
        ",
    )
    .bind(recipe_id)
    .bind(note)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(UpdateRecipeNotes, "/api/recipenotes/update")]
pub async fn update_recipe_notes(note_id: i32, note: String) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "UPDATE recipe_notes
            SET
                note = ?
            WHERE
                note_id = ?
        ",
    )
    .bind(note)
    .bind(note_id)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.last_insert_rowid().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(DeleteRecipeNote, "/api/recipenotes/delete")]
pub async fn delete_notes(note_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if note_id == 0 {
        return Err(ServerFnError::ServerError(String::from("Invalid note_id")));
    };

    let delete_row_result = sqlx::query("DELETE FROM recipe_notes WHERE note_id = ?")
        .bind(note_id)
        .execute(&*pool)
        .await;

    match delete_row_result {
        Ok(_) => Ok(()),
        Err(Error::RowNotFound) => Err(ServerFnError::ServerError(String::from(
            "Recipe note not found",
        ))),
        Err(err) => Err(ServerFnError::ServerError(format!(
            "Failed to delete note: {err}"
        ))),
    }
}
