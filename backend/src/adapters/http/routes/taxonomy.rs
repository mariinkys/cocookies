//! Categories and difficulties share these handlers; the router of each is built with its
//! [`TaxonomyKind`], which handlers receive as an `Extension`.
//!
//! | method | path         | notes                                   |
//! |--------|--------------|-----------------------------------------|
//! | GET    | `/`          | paged, `sortBy` = name/createdAt/...    |
//! | GET    | `/selector`  | every item, for dropdowns               |
//! | GET    | `/{id}`      | owner only                              |
//! | POST   | `/`          | name unique per user                    |
//! | PUT    | `/{id}`      | owner only                              |
//! | DELETE | `/{id}`      | owner only; recipes keep existing       |

use std::sync::Arc;

use axum::{Extension, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        extractors::{AuthUser, Json, Path, Query},
        pagination::{PageParams, PageResponse},
    },
    app_error::AppResult,
    entities::taxonomy::{TaxonomyDraft, TaxonomyItem, TaxonomyKind},
    use_cases::taxonomy::TaxonomyUseCases,
};

pub fn router(kind: TaxonomyKind) -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/selector", get(selector))
        .route("/{id}", get(get_one).put(update).delete(delete))
        .layer(Extension(kind))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyResponse {
    id: String,
    user_id: String,
    name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<TaxonomyItem> for TaxonomyResponse {
    fn from(i: TaxonomyItem) -> Self {
        Self {
            id: i.id,
            user_id: i.user_id,
            name: i.name,
            description: i.description,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TaxonomyPayload {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: Option<String>,
}

impl From<TaxonomyPayload> for TaxonomyDraft {
    fn from(p: TaxonomyPayload) -> Self {
        Self {
            name: p.name,
            description: p.description,
        }
    }
}

type Taxonomy = State<Arc<TaxonomyUseCases>>;
type Kind = Extension<TaxonomyKind>;

async fn list(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
    Query(params): Query<PageParams>,
) -> AppResult<axum::Json<PageResponse<TaxonomyResponse>>> {
    let page = taxonomy.page(&me, kind, params.into_query()?).await?;
    Ok(axum::Json(page.map(Into::into).into()))
}

async fn selector(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
) -> AppResult<axum::Json<Vec<TaxonomyResponse>>> {
    let items = taxonomy.list(&me, kind).await?;
    Ok(axum::Json(items.into_iter().map(Into::into).collect()))
}

async fn get_one(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<axum::Json<TaxonomyResponse>> {
    Ok(axum::Json(
        taxonomy.get(&me, kind, &id.to_string()).await?.into(),
    ))
}

async fn create(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
    Json(payload): Json<TaxonomyPayload>,
) -> AppResult<(StatusCode, axum::Json<TaxonomyResponse>)> {
    let item = taxonomy.create(&me, kind, payload.into()).await?;
    Ok((StatusCode::CREATED, axum::Json(item.into())))
}

async fn update(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<TaxonomyPayload>,
) -> AppResult<axum::Json<TaxonomyResponse>> {
    Ok(axum::Json(
        taxonomy
            .update(&me, kind, &id.to_string(), payload.into())
            .await?
            .into(),
    ))
}

async fn delete(
    State(taxonomy): Taxonomy,
    Extension(kind): Kind,
    AuthUser(me): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    taxonomy.delete(&me, kind, &id.to_string()).await?;
    Ok(StatusCode::NO_CONTENT)
}
