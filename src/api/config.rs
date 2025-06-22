use leptos::prelude::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Error, Pool, Row, Sqlite};
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::{models::config::Config, utils::pdf_themes::PdfTheme};

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

#[server(TestConfig, "/api/config/test")]
pub async fn test_config() -> Result<(), ServerFnError> {
    let config = get_config()
        .await
        .map_err(|err| ServerFnError::new(format!("Failed to get config: {err}")))?;

    if config
        .gotenberg_location
        .as_ref()
        .is_none_or(|loc| loc.is_empty())
    {
        return Err(ServerFnError::new("No Gotenberg location found"));
    }

    let client = gotenberg_pdf::Client::new(&config.gotenberg_location.unwrap());
    let result = client.health_check().await;

    match result {
        Ok(health) => match health.status {
            gotenberg_pdf::health::HealthStatus::Up => Ok(()),
            gotenberg_pdf::health::HealthStatus::Down => {
                Err(ServerFnError::new("Gotenberg Health is Down"))
            }
        },
        Err(err) => {
            eprintln!("{err}");
            Err(ServerFnError::new(format!("Gotenberg Error: {err}")))
        }
    }
}

// #[server(ExportPdf, "/api/config/exportpdf")]
// pub async fn export_pdf(
//     body_html: String,
//     recipe_image_path: String,
// ) -> Result<String, ServerFnError> {
//     use base64::{Engine as _, engine::general_purpose::STANDARD};

//     let config = get_config()
//         .await
//         .map_err(|err| ServerFnError::new(format!("Failed to get config: {err}")))?;

//     if config
//         .gotenberg_location
//         .as_ref()
//         .is_none_or(|loc| loc.is_empty())
//     {
//         return Err(ServerFnError::new(
//             "No Gotenberg location found".to_string(),
//         ));
//     }

//     //  let static_dir = std::env::var("LEPTOS_SITE_ROOT")
//     //     .or_else(|_| std::env::var("STATIC_DIR"))
//     //     .unwrap_or_else(|_| "target/site".to_string());

//     // let css_path = format!("{}/pkg/cocookies.css", static_dir);
//     // let css = std::fs::read_to_string(&css_path)
//     //     .map_err(|err| ServerFnError::new(format!("Failed to read CSS file: {err}")))?;

//     let current_dir = std::env::current_dir()
//         .map_err(|err| ServerFnError::new(format!("Failed to get current directory: {err}")))?;
//     let css_path = current_dir.join("target/site/pkg/cocookies.css");
//     let css = std::fs::read_to_string(&css_path).map_err(|err| {
//         ServerFnError::new(format!("Failed to read CSS from {css_path:?}: {err}"))
//     })?;

//     let base64_image = if let Ok(image) = std::fs::read(&recipe_image_path) {
//         STANDARD.encode(&image)
//     } else {
//         return Err(ServerFnError::new(format!(
//             "Can't encode image with path: {recipe_image_path}"
//         )));
//     };

//     let html = format!(
//         r#"<!DOCTYPE html>
//             <html lang="en">
//                 <head>
//                     <meta charset="UTF-8">
//                     <style>{css}</style>
//                 </head>
//                 <body>
//                     {body_html}
//                 </body>
//             </html>"#
//     );

//     let client = gotenberg_pdf::Client::new(&config.gotenberg_location.unwrap());
//     let result = client
//         .pdf_from_html(&html, gotenberg_pdf::WebOptions::default())
//         .await;

//     match result {
//         Ok(bytes) => Ok(STANDARD.encode(&bytes)),
//         Err(err) => {
//             eprintln!("{err}");
//             Err(ServerFnError::new(format!("Gotenberg Error: {err}")))
//         }
//     }
// }

#[server(ExportPdf, "/api/config/exportpdf")]
pub async fn export_pdf(recipe_id: i32, theme: Option<PdfTheme>) -> Result<String, ServerFnError> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    let config = get_config()
        .await
        .map_err(|err| ServerFnError::new(format!("Failed to get config: {err}")))?;

    if config
        .gotenberg_location
        .as_ref()
        .is_none_or(|loc| loc.is_empty())
    {
        return Err(ServerFnError::new("No Gotenberg location found"));
    }

    let recipe = crate::api::recipe::get_recipe(recipe_id)
        .await
        .unwrap_or_default();
    let recipe_ingredients = crate::api::recipe_ingredients::get_all_recipe_ingredients(recipe_id)
        .await
        .unwrap_or_default();
    let recipe_steps = crate::api::recipe_steps::get_all_recipe_steps(recipe_id)
        .await
        .unwrap_or_default();
    let recipe_notes = crate::api::recipe_notes::get_all_recipe_notes(recipe_id)
        .await
        .unwrap_or_default();

    let recipe_html = format!(
        r#"<h1>{}</h1>
        <p class="recipe-meta">{} ({} servings, {}min)</p>
    "#,
        recipe.name,
        recipe.description.unwrap_or_default(),
        recipe.servings.unwrap_or_default(),
        recipe.prep_time_minutes.unwrap_or_default()
    );

    let mut ingredients_html = String::from("<h1>Ingredients</h1>");
    if recipe_ingredients.is_empty() {
        ingredients_html += "<p>No Ingredients...</p>"
    } else {
        for ingredient in recipe_ingredients {
            if ingredient.quantity.is_some() || ingredient.unit.is_some() {
                ingredients_html += &format!(
                    r#"<p class="ingredient">{} {} - {}</p>"#,
                    ingredient.quantity.unwrap_or_default(),
                    ingredient.unit.unwrap_or_default(),
                    ingredient.ingredient_name
                );
            } else {
                ingredients_html += &format!(
                    r#"<p class="ingredient">{}</p>"#,
                    ingredient.ingredient_name
                );
            }
        }
    }

    let mut steps_html = String::from("<h1>Steps</h1>");
    if recipe_steps.is_empty() {
        steps_html += "<p>No Steps...</p>"
    } else {
        for step in recipe_steps {
            steps_html += &format!(r#"<p class="step">{}</p>"#, step.instructions);
        }
    }

    let mut notes_html = String::from("<h1>Notes</h1>");
    if recipe_notes.is_empty() {
        notes_html += "<p>No Notes...</p>"
    } else {
        for note in recipe_notes {
            notes_html += &format!(r#"<p class="note">{}</p>"#, note.note);
        }
    }

    let style = theme.unwrap_or_default().get_style();

    let result_html = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>Recipe Export</title>
                {style}
            </head>
            <body>
                {recipe_html}
                {ingredients_html}
                {steps_html}
                {notes_html}
                <footer><a href="https://github.com/mariinkys/cocookies" target="_blank">Made with Cocookies ❤️</a></footer>
            </body>
        </html>"#
    );

    let client = gotenberg_pdf::Client::new(&config.gotenberg_location.unwrap());
    let result = client
        .pdf_from_html(&result_html, gotenberg_pdf::WebOptions::default())
        .await;

    match result {
        Ok(bytes) => Ok(STANDARD.encode(&bytes)),
        Err(err) => {
            eprintln!("{err}");
            Err(ServerFnError::new(format!("Gotenberg Error: {err}")))
        }
    }
}
