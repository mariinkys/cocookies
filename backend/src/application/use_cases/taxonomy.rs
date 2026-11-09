//! Categories and difficulties: private to their owner, names unique per user.

use std::sync::Arc;

use async_trait::async_trait;

use super::user::unique_as;
use crate::{
    app_error::{AppError, AppResult},
    entities::{
        page::{Page, PageQuery},
        session::Identity,
        taxonomy::{TaxonomyDraft, TaxonomyItem, TaxonomyKind, TaxonomySort},
    },
};

#[async_trait]
pub trait TaxonomyPersistence: Send + Sync {
    async fn page_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        query: &PageQuery<TaxonomySort>,
    ) -> AppResult<(Vec<TaxonomyItem>, i64)>;
    async fn list_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
    ) -> AppResult<Vec<TaxonomyItem>>;
    async fn find_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<Option<TaxonomyItem>>;
    /// Does `user_id` own an item called `name` (other than `except_id`)?
    async fn taxonomy_name_taken(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        name: &str,
        except_id: Option<&str>,
    ) -> AppResult<bool>;
    async fn create_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<TaxonomyItem>;
    async fn update_taxonomy(
        &self,
        kind: TaxonomyKind,
        id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<()>;
    /// Recipes referencing the item keep existing (the reference is cleared).
    async fn delete_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<()>;
    async fn taxonomy_owned_by(
        &self,
        kind: TaxonomyKind,
        id: &str,
        user_id: &str,
    ) -> AppResult<bool>;
}

#[derive(Clone)]
pub struct TaxonomyUseCases {
    persistence: Arc<dyn TaxonomyPersistence>,
}

impl TaxonomyUseCases {
    pub fn new(persistence: Arc<dyn TaxonomyPersistence>) -> Self {
        Self { persistence }
    }

    pub async fn page(
        &self,
        me: &Identity,
        kind: TaxonomyKind,
        query: PageQuery<TaxonomySort>,
    ) -> AppResult<Page<TaxonomyItem>> {
        let (items, total) = self
            .persistence
            .page_taxonomy(kind, &me.user_id, &query)
            .await?;
        Ok(Page::new(items, &query, total))
    }

    pub async fn list(&self, me: &Identity, kind: TaxonomyKind) -> AppResult<Vec<TaxonomyItem>> {
        self.persistence.list_taxonomy(kind, &me.user_id).await
    }

    pub async fn get(
        &self,
        me: &Identity,
        kind: TaxonomyKind,
        id: &str,
    ) -> AppResult<TaxonomyItem> {
        self.load_owned(me, kind, id).await
    }

    pub async fn create(
        &self,
        me: &Identity,
        kind: TaxonomyKind,
        draft: TaxonomyDraft,
    ) -> AppResult<TaxonomyItem> {
        draft.validate()?;
        let draft = draft.normalized();
        if self
            .persistence
            .taxonomy_name_taken(kind, &me.user_id, &draft.name, None)
            .await?
        {
            return Err(duplicate(kind, &draft.name));
        }
        self.persistence
            .create_taxonomy(kind, &me.user_id, &draft)
            .await
            .map_err(|e| unique_as(e, || duplicate(kind, &draft.name)))
    }

    pub async fn update(
        &self,
        me: &Identity,
        kind: TaxonomyKind,
        id: &str,
        draft: TaxonomyDraft,
    ) -> AppResult<TaxonomyItem> {
        self.load_owned(me, kind, id).await?;
        draft.validate()?;
        let draft = draft.normalized();
        if self
            .persistence
            .taxonomy_name_taken(kind, &me.user_id, &draft.name, Some(id))
            .await?
        {
            return Err(duplicate(kind, &draft.name));
        }
        self.persistence
            .update_taxonomy(kind, id, &draft)
            .await
            .map_err(|e| unique_as(e, || duplicate(kind, &draft.name)))?;
        self.load_owned(me, kind, id).await
    }

    pub async fn delete(&self, me: &Identity, kind: TaxonomyKind, id: &str) -> AppResult<()> {
        self.load_owned(me, kind, id).await?;
        self.persistence.delete_taxonomy(kind, id).await
    }

    async fn load_owned(
        &self,
        me: &Identity,
        kind: TaxonomyKind,
        id: &str,
    ) -> AppResult<TaxonomyItem> {
        let item = self
            .persistence
            .find_taxonomy(kind, id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("{} not found with id: {id}", kind.label()))
            })?;
        if !me.owns(&item.user_id) {
            return Err(AppError::Forbidden(format!(
                "You can only access your own {}",
                kind.label().to_lowercase()
            )));
        }
        Ok(item)
    }
}

fn duplicate(kind: TaxonomyKind, name: &str) -> AppError {
    AppError::Conflict(format!(
        "{} with name '{name}' already exists",
        kind.label()
    ))
}
