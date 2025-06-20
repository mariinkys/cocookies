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

#[server(ExportPdf, "/api/config/exportpdf")]
pub async fn export_pdf(body_html: String) -> Result<String, ServerFnError> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    let config = get_config()
        .await
        .map_err(|err| ServerFnError::new(format!("Failed to get config: {err}")))?;

    if config
        .gotenberg_location
        .as_ref()
        .is_none_or(|loc| loc.is_empty())
    {
        return Err(ServerFnError::new(
            "No Gotenberg location found".to_string(),
        ));
    }

    //  let static_dir = std::env::var("LEPTOS_SITE_ROOT")
    //     .or_else(|_| std::env::var("STATIC_DIR"))
    //     .unwrap_or_else(|_| "target/site".to_string());

    // let css_path = format!("{}/pkg/cocookies.css", static_dir);
    // let css = std::fs::read_to_string(&css_path)
    //     .map_err(|err| ServerFnError::new(format!("Failed to read CSS file: {err}")))?;

    let current_dir = std::env::current_dir()
        .map_err(|err| ServerFnError::new(format!("Failed to get current directory: {err}")))?;
    let css_path = current_dir.join("target/site/pkg/cocookies.css");
    let css = std::fs::read_to_string(&css_path).map_err(|err| {
        ServerFnError::new(format!("Failed to read CSS from {css_path:?}: {err}"))
    })?;

    let html = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <style>{css}</style>
                </head>
                <body>
                    {body_html}
                </body>
            </html>"#
    );

    let client = gotenberg_pdf::Client::new(&config.gotenberg_location.unwrap());
    let result = client
        .pdf_from_html(&html, gotenberg_pdf::WebOptions::default())
        .await;

    match result {
        Ok(bytes) => Ok(STANDARD.encode(&bytes)),
        Err(err) => {
            eprintln!("{err}");
            Err(ServerFnError::new(format!("Gotenberg Error: {err}")))
        }
    }
}
