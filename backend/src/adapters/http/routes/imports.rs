//! AI-assisted recipe import.
//!
//! | method | path                     | body                                    |
//! |--------|--------------------------|-----------------------------------------|
//! | POST   | `/analyze/file`          | multipart: `file` (PDF/image) + `data`  |
//! | POST   | `/analyze/url?url=…`     | `{"prompt": "..."}`                     |
//!
//! Both answer `{"recipe": {...}}` shaped like a recipe request, ready to pre-fill the form.

use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State, multipart::MultipartRejection},
    routing::post,
};
use serde::{Deserialize, Serialize};

use super::recipes::NutrientValuesDto;
use crate::{
    adapters::http::{
        app_state::AppState,
        extractors::{AuthUser, Json, Query},
    },
    app_error::{AppError, AppResult},
    entities::recipe::RecipeDraft,
    use_cases::recipe_import::RecipeImportUseCases,
};

/// Uploads (photos of recipe books, PDFs) may be a few megabytes.
const MAX_UPLOAD_BYTES: usize = 20 * 1024 * 1024;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/analyze/file",
            post(analyze_file).layer(DefaultBodyLimit::max(MAX_UPLOAD_BYTES)),
        )
        .route("/analyze/url", post(analyze_url))
}

#[derive(Debug, Deserialize)]
struct AnalyzePayload {
    #[serde(default)]
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct UrlParams {
    url: String,
}

#[derive(Debug, Serialize)]
struct ImportResponse {
    recipe: ImportedRecipeResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportedRecipeResponse {
    title: String,
    description: Option<String>,
    prep_time: Option<i64>,
    cook_time: Option<i64>,
    servings: i64,
    image_url: Option<String>,
    steps: Vec<ImportedStep>,
    ingredients: Vec<ImportedIngredient>,
    nutrition: Option<ImportedNutrition>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportedStep {
    step_number: i64,
    instructions: String,
    duration: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportedIngredient {
    name: String,
    quantity: Option<f64>,
    unit: Option<String>,
    notes: Option<String>,
    sort_order: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportedNutrition {
    serving_size_value: Option<f64>,
    serving_size_unit: String,
    #[serde(flatten)]
    values: NutrientValuesDto,
}

impl From<RecipeDraft> for ImportResponse {
    fn from(d: RecipeDraft) -> Self {
        Self {
            recipe: ImportedRecipeResponse {
                title: d.title,
                description: d.description,
                prep_time: d.prep_time,
                cook_time: d.cook_time,
                servings: d.servings,
                image_url: d.image_url,
                steps: d
                    .steps
                    .into_iter()
                    .map(|s| ImportedStep {
                        step_number: s.step_number,
                        instructions: s.instructions,
                        duration: s.duration,
                    })
                    .collect(),
                ingredients: d
                    .ingredients
                    .into_iter()
                    .map(|i| ImportedIngredient {
                        name: i.name,
                        quantity: i.quantity,
                        unit: i.unit,
                        notes: i.notes,
                        sort_order: i.sort_order,
                    })
                    .collect(),
                nutrition: d.nutrition.map(|n| ImportedNutrition {
                    serving_size_value: n.serving_size_value,
                    serving_size_unit: n.serving_size_unit,
                    values: n.values.into(),
                }),
            },
        }
    }
}

/// `multipart/form-data` with a `file` part and a `data` part holding `{"prompt": "..."}`.
async fn analyze_file(
    State(imports): State<Arc<RecipeImportUseCases>>,
    _me: AuthUser,
    multipart: Result<Multipart, MultipartRejection>,
) -> AppResult<axum::Json<ImportResponse>> {
    let mut multipart = multipart.map_err(|e| AppError::Validation(e.body_text()))?;
    let mut file: Option<(Vec<u8>, Option<String>)> = None;
    let mut payload: Option<AnalyzePayload> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(e.body_text()))?
    {
        match field.name() {
            Some("file") => {
                let content_type = field.content_type().map(str::to_owned);
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::Validation(e.body_text()))?;
                file = Some((bytes.to_vec(), content_type));
            }
            Some("data") => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::Validation(e.body_text()))?;
                payload = Some(
                    serde_json::from_slice(&bytes)
                        .map_err(|e| AppError::Validation(format!("Invalid 'data' part: {e}")))?,
                );
            }
            _ => {}
        }
    }

    let (bytes, content_type) =
        file.ok_or_else(|| AppError::Validation("A 'file' part is required".into()))?;
    let prompt = payload.map(|p| p.prompt).unwrap_or_default();
    let draft = imports
        .import_file(bytes, content_type.as_deref(), &prompt)
        .await?;
    Ok(axum::Json(draft.into()))
}

async fn analyze_url(
    State(imports): State<Arc<RecipeImportUseCases>>,
    _me: AuthUser,
    Query(params): Query<UrlParams>,
    Json(payload): Json<AnalyzePayload>,
) -> AppResult<axum::Json<ImportResponse>> {
    let draft = imports.import_url(&params.url, &payload.prompt).await?;
    Ok(axum::Json(draft.into()))
}
