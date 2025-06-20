use leptos::prelude::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Error, Pool, Row, Sqlite};
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::models::config::Config;

#[server(GetConfig, "/api/config")]
pub async fn get_config() -> Result<Config, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let row_result = sqlx::query("SELECT * FROM config WHERE id = ?")
        .bind(1)
        .fetch_one(&*pool)
        .await;

    match row_result {
        Ok(row) => {
            let config: Config = Config {
                id: row.get("id"),
                gotenberg_location: row.get("gotenberg_location"),
            };
            Ok(config)
        }
        Err(Error::RowNotFound) => Err(ServerFnError::new(String::from("Config not found"))),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}

#[server(UpdateConfig, "/api/config/update")]
pub async fn update_config(gotenberg_location: String) -> Result<i32, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let command_res = sqlx::query(
        "UPDATE config
            SET
                gotenberg_location = ?
            WHERE
                id = ?
        ",
    )
    .bind(gotenberg_location)
    .bind(1)
    .execute(&*pool)
    .await;

    match command_res {
        Ok(result) => Ok(result.rows_affected().try_into().unwrap()),
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
    }
}
