//! | method | path                        | who                                    |
//! |--------|-----------------------------|----------------------------------------|
//! | GET    | `/`                         | signed-in: own recipes (cards)         |
//! | GET    | `/shared`                   | users and guests: shared (cards)       |
//! | GET    | `/{id}`                     | owner, or anyone if shared             |
//! | GET    | `/{id}/pdf/{template}`      | same as above; `default-en` / `-es`    |
//! | POST   | `/`                         | signed-in                              |
//! | PUT    | `/{id}`                     | owner                                  |
//! | DELETE | `/{id}`                     | owner                                  |

use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::taxonomy::TaxonomyResponse;
use crate::{
    adapters::http::{
        app_state::AppState,
        extractors::{AuthUser, Json, OptionalUser, Path, Query},
        pagination::{PageParams, PageResponse},
    },
    app_error::AppResult,
    entities::recipe::{
        Ingredient, IngredientDraft, NamedRef, NutrientValues, Nutrition, NutritionDraft,
        PdfTemplate, Recipe, RecipeDraft, RecipeSummary, Step, StepDraft,
    },
    use_cases::recipe::RecipeUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_own).post(create))
        .route("/shared", get(list_shared))
        .route("/{id}", get(get_one).put(update).delete(delete))
        .route("/{id}/pdf/{template}", get(pdf))
}

// ---------------------------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecipeResponse {
    id: String,
    user_id: String,
    category: Option<TaxonomyResponse>,
    difficulty: Option<TaxonomyResponse>,
    title: String,
    description: Option<String>,
    prep_time: Option<i64>,
    cook_time: Option<i64>,
    servings: i64,
    image_url: Option<String>,
    shared: bool,
    steps: Vec<StepResponse>,
    ingredients: Vec<IngredientResponse>,
    nutrition: Option<NutritionResponse>,
    created_at: String,
    updated_at: String,
}

/// Lightweight card used by listings.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecipeSummaryResponse {
    id: String,
    user_id: String,
    category: Option<NamedRefResponse>,
    difficulty: Option<NamedRefResponse>,
    title: String,
    description: Option<String>,
    prep_time: Option<i64>,
    cook_time: Option<i64>,
    servings: i64,
    image_url: Option<String>,
    shared: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
struct NamedRefResponse {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StepResponse {
    id: i64,
    step_number: i64,
    instructions: String,
    duration: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct IngredientResponse {
    id: i64,
    name: String,
    quantity: Option<f64>,
    unit: Option<String>,
    notes: Option<String>,
    sort_order: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutritionResponse {
    id: i64,
    serving_size_value: f64,
    serving_size_unit: String,
    #[serde(flatten)]
    values: NutrientValuesDto,
}

/// Nutrients per serving size, shared by requests and responses.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub(super) struct NutrientValuesDto {
    calories: Option<f64>,
    protein_g: Option<f64>,
    carbs_g: Option<f64>,
    sugar_g: Option<f64>,
    fat_g: Option<f64>,
    saturated_fat_g: Option<f64>,
    fiber_g: Option<f64>,
    sodium_mg: Option<f64>,
}

impl From<NutrientValues> for NutrientValuesDto {
    fn from(v: NutrientValues) -> Self {
        Self {
            calories: v.calories,
            protein_g: v.protein_g,
            carbs_g: v.carbs_g,
            sugar_g: v.sugar_g,
            fat_g: v.fat_g,
            saturated_fat_g: v.saturated_fat_g,
            fiber_g: v.fiber_g,
            sodium_mg: v.sodium_mg,
        }
    }
}

impl From<NutrientValuesDto> for NutrientValues {
    fn from(v: NutrientValuesDto) -> Self {
        Self {
            calories: v.calories,
            protein_g: v.protein_g,
            carbs_g: v.carbs_g,
            sugar_g: v.sugar_g,
            fat_g: v.fat_g,
            saturated_fat_g: v.saturated_fat_g,
            fiber_g: v.fiber_g,
            sodium_mg: v.sodium_mg,
        }
    }
}

impl From<Recipe> for RecipeResponse {
    fn from(r: Recipe) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            category: r.category.map(Into::into),
            difficulty: r.difficulty.map(Into::into),
            title: r.title,
            description: r.description,
            prep_time: r.prep_time,
            cook_time: r.cook_time,
            servings: r.servings,
            image_url: r.image_url,
            shared: r.shared,
            steps: r.steps.into_iter().map(Into::into).collect(),
            ingredients: r.ingredients.into_iter().map(Into::into).collect(),
            nutrition: r.nutrition.map(Into::into),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl From<RecipeSummary> for RecipeSummaryResponse {
    fn from(r: RecipeSummary) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            category: r.category.map(Into::into),
            difficulty: r.difficulty.map(Into::into),
            title: r.title,
            description: r.description,
            prep_time: r.prep_time,
            cook_time: r.cook_time,
            servings: r.servings,
            image_url: r.image_url,
            shared: r.shared,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl From<NamedRef> for NamedRefResponse {
    fn from(n: NamedRef) -> Self {
        Self {
            id: n.id,
            name: n.name,
        }
    }
}

impl From<Step> for StepResponse {
    fn from(s: Step) -> Self {
        Self {
            id: s.id,
            step_number: s.step_number,
            instructions: s.instructions,
            duration: s.duration,
        }
    }
}

impl From<Ingredient> for IngredientResponse {
    fn from(i: Ingredient) -> Self {
        Self {
            id: i.id,
            name: i.name,
            quantity: i.quantity,
            unit: i.unit,
            notes: i.notes,
            sort_order: i.sort_order,
        }
    }
}

impl From<Nutrition> for NutritionResponse {
    fn from(n: Nutrition) -> Self {
        Self {
            id: n.id,
            serving_size_value: n.serving_size_value,
            serving_size_unit: n.serving_size_unit,
            values: n.values.into(),
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecipePayload {
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default, deserialize_with = "lenient_uuid")]
    category_id: Option<Uuid>,
    #[serde(default, deserialize_with = "lenient_uuid")]
    difficulty_id: Option<Uuid>,
    #[serde(default)]
    prep_time: Option<i64>,
    #[serde(default)]
    cook_time: Option<i64>,
    #[serde(default)]
    servings: i64,
    #[serde(default)]
    image_url: Option<String>,
    #[serde(default)]
    shared: bool,
    #[serde(default)]
    steps: Vec<StepPayload>,
    #[serde(default)]
    ingredients: Vec<IngredientPayload>,
    #[serde(default)]
    nutrition: Option<NutritionPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StepPayload {
    #[serde(default)]
    step_number: i64,
    #[serde(default)]
    instructions: String,
    #[serde(default)]
    duration: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngredientPayload {
    #[serde(default)]
    name: String,
    #[serde(default)]
    quantity: Option<f64>,
    #[serde(default)]
    unit: Option<String>,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    sort_order: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutritionPayload {
    #[serde(default)]
    serving_size_value: Option<f64>,
    #[serde(default)]
    serving_size_unit: String,
    #[serde(flatten)]
    values: NutrientValuesDto,
}

impl From<RecipePayload> for RecipeDraft {
    fn from(p: RecipePayload) -> Self {
        Self {
            title: p.title,
            description: p.description,
            category_id: p.category_id.map(|u| u.to_string()),
            difficulty_id: p.difficulty_id.map(|u| u.to_string()),
            prep_time: p.prep_time,
            cook_time: p.cook_time,
            servings: p.servings,
            image_url: p.image_url,
            shared: p.shared,
            steps: p
                .steps
                .into_iter()
                .map(|s| StepDraft {
                    step_number: s.step_number,
                    instructions: s.instructions,
                    duration: s.duration,
                })
                .collect(),
            ingredients: p
                .ingredients
                .into_iter()
                .map(|i| IngredientDraft {
                    name: i.name,
                    quantity: i.quantity,
                    unit: i.unit,
                    notes: i.notes,
                    sort_order: i.sort_order,
                })
                .collect(),
            nutrition: p.nutrition.map(|n| NutritionDraft {
                serving_size_value: n.serving_size_value,
                serving_size_unit: n.serving_size_unit,
                values: n.values.into(),
            }),
        }
    }
}

/// Accepts a UUID, `null`, or an empty string (treated as `null`).
fn lenient_uuid<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Uuid>, D::Error> {
    match Option::<String>::deserialize(d)?.as_deref().map(str::trim) {
        None | Some("") => Ok(None),
        Some(s) => Uuid::parse_str(s)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

// ---------------------------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------------------------

type Recipes = State<Arc<RecipeUseCases>>;

async fn list_own(
    State(recipes): Recipes,
    AuthUser(me): AuthUser,
    Query(params): Query<PageParams>,
) -> AppResult<axum::Json<PageResponse<RecipeSummaryResponse>>> {
    let page = recipes.list_own(&me, params.into_query()?).await?;
    Ok(axum::Json(page.map(Into::into).into()))
}

async fn list_shared(
    State(recipes): Recipes,
    OptionalUser(viewer): OptionalUser,
    Query(params): Query<PageParams>,
) -> AppResult<axum::Json<PageResponse<RecipeSummaryResponse>>> {
    let page = recipes
        .list_shared(viewer.as_ref(), params.into_query()?)
        .await?;
    Ok(axum::Json(page.map(Into::into).into()))
}

async fn get_one(
    State(recipes): Recipes,
    OptionalUser(viewer): OptionalUser,
    Path(id): Path<Uuid>,
) -> AppResult<axum::Json<RecipeResponse>> {
    Ok(axum::Json(
        recipes.get(viewer.as_ref(), &id.to_string()).await?.into(),
    ))
}

async fn create(
    State(recipes): Recipes,
    AuthUser(me): AuthUser,
    Json(payload): Json<RecipePayload>,
) -> AppResult<(StatusCode, axum::Json<RecipeResponse>)> {
    let recipe = recipes.create(&me, payload.into()).await?;
    Ok((StatusCode::CREATED, axum::Json(recipe.into())))
}

async fn update(
    State(recipes): Recipes,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<RecipePayload>,
) -> AppResult<axum::Json<RecipeResponse>> {
    Ok(axum::Json(
        recipes
            .update(&me, &id.to_string(), payload.into())
            .await?
            .into(),
    ))
}

async fn delete(
    State(recipes): Recipes,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    recipes.delete(&me, &id.to_string()).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn pdf(
    State(recipes): Recipes,
    OptionalUser(viewer): OptionalUser,
    Path((id, template)): Path<(Uuid, String)>,
) -> AppResult<Response> {
    let bytes = recipes
        .pdf(
            viewer.as_ref(),
            &id.to_string(),
            PdfTemplate::parse(&template),
        )
        .await?;
    let disposition = format!("attachment; filename=\"recipe-{id}.pdf\"");
    Ok((
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/pdf"),
            ),
            (
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&disposition)
                    .unwrap_or(HeaderValue::from_static("attachment")),
            ),
        ],
        bytes,
    )
        .into_response())
}
