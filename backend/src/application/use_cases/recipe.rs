//! Recipe rules.
//!
//! * Anyone allowed to view — signed-in users, and anonymous guests when guest access is
//!   enabled — can read **shared** recipes; private recipes are visible to their owner only.
//! * Only the owner can modify or delete a recipe, shared or not.
//! * A recipe can only reference the owner's own categories and difficulties.

use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use super::taxonomy::TaxonomyPersistence;
use crate::{
    app_error::{AppError, AppResult},
    entities::{
        page::{Page, PageQuery},
        recipe::{PdfTemplate, Recipe, RecipeDraft, RecipeScope, RecipeSort, RecipeSummary},
        session::Identity,
        taxonomy::TaxonomyKind,
    },
};

#[async_trait]
pub trait RecipePersistence: Send + Sync {
    async fn page_recipes(
        &self,
        scope: &RecipeScope,
        query: &PageQuery<RecipeSort>,
    ) -> AppResult<(Vec<RecipeSummary>, i64)>;
    async fn find_recipe(&self, id: &str) -> AppResult<Option<Recipe>>;
    async fn recipe_owner(&self, id: &str) -> AppResult<Option<String>>;
    /// Stores a new recipe with its children; returns its id.
    async fn create_recipe(&self, user_id: &str, draft: &RecipeDraft) -> AppResult<String>;
    /// Replaces the recipe and all its children.
    async fn update_recipe(&self, id: &str, draft: &RecipeDraft) -> AppResult<()>;
    async fn delete_recipe(&self, id: &str) -> AppResult<()>;
}

/// Turns a recipe into a printable document.
#[async_trait]
pub trait RecipeRenderer: Send + Sync {
    /// `image` holds the raw bytes of the recipe picture, when available.
    async fn render_pdf(
        &self,
        recipe: Recipe,
        template: PdfTemplate,
        image: Option<Vec<u8>>,
    ) -> AppResult<Vec<u8>>;
}

/// A resource downloaded from a user-supplied URL.
#[derive(Debug, Clone)]
pub struct RemoteResource {
    pub body: Vec<u8>,
    pub content_type: Option<String>,
}

/// Downloads user-supplied URLs (recipe images, pages to import). Implementations must refuse
/// to reach private networks, since the URL is attacker-controlled.
#[async_trait]
pub trait RemoteFetcher: Send + Sync {
    /// Fails with [`AppError::Unprocessable`] explaining why the resource is unavailable.
    async fn fetch(&self, url: &str, max_bytes: usize) -> AppResult<RemoteResource>;
}

#[derive(Debug, Clone)]
pub struct RecipePolicy {
    /// Let anonymous visitors read shared recipes.
    pub guest_access: bool,
}

/// Largest remote image we are willing to embed in a PDF.
const MAX_PDF_IMAGE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone)]
pub struct RecipeUseCases {
    recipes: Arc<dyn RecipePersistence>,
    taxonomy: Arc<dyn TaxonomyPersistence>,
    fetcher: Arc<dyn RemoteFetcher>,
    renderer: Arc<dyn RecipeRenderer>,
    policy: RecipePolicy,
}

impl RecipeUseCases {
    pub fn new(
        recipes: Arc<dyn RecipePersistence>,
        taxonomy: Arc<dyn TaxonomyPersistence>,
        fetcher: Arc<dyn RemoteFetcher>,
        renderer: Arc<dyn RecipeRenderer>,
        policy: RecipePolicy,
    ) -> Self {
        Self {
            recipes,
            taxonomy,
            fetcher,
            renderer,
            policy,
        }
    }

    pub async fn list_own(
        &self,
        me: &Identity,
        query: PageQuery<RecipeSort>,
    ) -> AppResult<Page<RecipeSummary>> {
        let scope = RecipeScope::OwnedBy(me.user_id.clone());
        let (items, total) = self.recipes.page_recipes(&scope, &query).await?;
        Ok(Page::new(items, &query, total))
    }

    pub async fn list_shared(
        &self,
        viewer: Option<&Identity>,
        query: PageQuery<RecipeSort>,
    ) -> AppResult<Page<RecipeSummary>> {
        self.ensure_can_browse(viewer)?;
        let (items, total) = self
            .recipes
            .page_recipes(&RecipeScope::Shared, &query)
            .await?;
        Ok(Page::new(items, &query, total))
    }

    pub async fn get(&self, viewer: Option<&Identity>, id: &str) -> AppResult<Recipe> {
        self.ensure_can_browse(viewer)?;
        let recipe = self
            .recipes
            .find_recipe(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if recipe.shared || viewer.is_some_and(|v| v.owns(&recipe.user_id)) {
            return Ok(recipe);
        }
        Err(match viewer {
            None => AppError::unauthenticated(),
            Some(_) => AppError::Forbidden("You can only access your own recipe".into()),
        })
    }

    #[instrument(skip(self, draft), fields(user_id = %me.user_id))]
    pub async fn create(&self, me: &Identity, draft: RecipeDraft) -> AppResult<Recipe> {
        draft.validate()?;
        let draft = draft.normalized();
        self.check_references(me, &draft).await?;
        let id = self.recipes.create_recipe(&me.user_id, &draft).await?;
        info!(recipe_id = %id, "recipe created");
        self.recipes
            .find_recipe(&id)
            .await?
            .ok_or_else(|| not_found(&id))
    }

    #[instrument(skip(self, draft), fields(user_id = %me.user_id))]
    pub async fn update(&self, me: &Identity, id: &str, draft: RecipeDraft) -> AppResult<Recipe> {
        self.ensure_owner(me, id).await?;
        draft.validate()?;
        let draft = draft.normalized();
        self.check_references(me, &draft).await?;
        self.recipes.update_recipe(id, &draft).await?;
        self.recipes
            .find_recipe(id)
            .await?
            .ok_or_else(|| not_found(id))
    }

    #[instrument(skip(self), fields(user_id = %me.user_id))]
    pub async fn delete(&self, me: &Identity, id: &str) -> AppResult<()> {
        self.ensure_owner(me, id).await?;
        self.recipes.delete_recipe(id).await
    }

    pub async fn pdf(
        &self,
        viewer: Option<&Identity>,
        id: &str,
        template: PdfTemplate,
    ) -> AppResult<Vec<u8>> {
        let recipe = self.get(viewer, id).await?;

        // The picture is optional: if it cannot be fetched the PDF is produced without it.
        let image = match recipe.image_url.as_deref() {
            Some(url) => self
                .fetcher
                .fetch(url, MAX_PDF_IMAGE_BYTES)
                .await
                .inspect_err(|e| info!(recipe_id = %id, error = %e, "recipe image skipped in PDF"))
                .ok()
                .map(|r| r.body),
            None => None,
        };
        self.renderer.render_pdf(recipe, template, image).await
    }

    fn ensure_can_browse(&self, viewer: Option<&Identity>) -> AppResult<()> {
        if viewer.is_none() && !self.policy.guest_access {
            return Err(AppError::unauthenticated());
        }
        Ok(())
    }

    async fn ensure_owner(&self, me: &Identity, id: &str) -> AppResult<()> {
        let owner = self
            .recipes
            .recipe_owner(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if me.owns(&owner) {
            Ok(())
        } else {
            Err(AppError::Forbidden(
                "You can only modify your own recipe".into(),
            ))
        }
    }

    async fn check_references(&self, me: &Identity, draft: &RecipeDraft) -> AppResult<()> {
        for (kind, id) in [
            (TaxonomyKind::Category, &draft.category_id),
            (TaxonomyKind::Difficulty, &draft.difficulty_id),
        ] {
            if let Some(id) = id
                && !self
                    .taxonomy
                    .taxonomy_owned_by(kind, id, &me.user_id)
                    .await?
            {
                return Err(AppError::Validation(format!(
                    "{} not found with id: {id}",
                    kind.label()
                )));
            }
        }
        Ok(())
    }
}

fn not_found(id: &str) -> AppError {
    AppError::NotFound(format!("Recipe not found with id: {id}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::recipe::{IngredientDraft, StepDraft},
        use_cases::fakes::{FakeFetcher, FakeRenderer, InMemory},
    };

    fn use_cases(store: &Arc<InMemory>, guest_access: bool) -> RecipeUseCases {
        RecipeUseCases::new(
            store.clone(),
            store.clone(),
            Arc::new(FakeFetcher),
            Arc::new(FakeRenderer),
            RecipePolicy { guest_access },
        )
    }

    fn identity(id: &str) -> Identity {
        Identity {
            user_id: id.into(),
            email: format!("{id}@example.com"),
        }
    }

    fn draft(shared: bool) -> RecipeDraft {
        RecipeDraft {
            title: "Flan".into(),
            servings: 4,
            shared,
            steps: vec![StepDraft {
                step_number: 1,
                instructions: "Bake".into(),
                duration: None,
            }],
            ingredients: vec![IngredientDraft {
                name: "Eggs".into(),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn visibility_rules() {
        let store = Arc::new(InMemory::default());
        let recipes = use_cases(&store, true);
        let (ana, bob) = (identity("ana"), identity("bob"));

        let shared = recipes.create(&ana, draft(true)).await.unwrap();
        let private = recipes.create(&ana, draft(false)).await.unwrap();

        // Guests and other users read shared recipes only.
        assert!(recipes.get(None, &shared.id).await.is_ok());
        assert!(recipes.get(Some(&bob), &shared.id).await.is_ok());
        assert!(matches!(
            recipes.get(None, &private.id).await,
            Err(AppError::Unauthorized(_))
        ));
        assert!(matches!(
            recipes.get(Some(&bob), &private.id).await,
            Err(AppError::Forbidden(_))
        ));
        assert!(recipes.get(Some(&ana), &private.id).await.is_ok());

        // Only the owner may modify, even shared recipes.
        assert!(matches!(
            recipes.update(&bob, &shared.id, draft(true)).await,
            Err(AppError::Forbidden(_))
        ));
        assert!(matches!(
            recipes.delete(&bob, &shared.id).await,
            Err(AppError::Forbidden(_))
        ));
        recipes.delete(&ana, &shared.id).await.unwrap();
        assert!(matches!(
            recipes.get(Some(&ana), &shared.id).await,
            Err(AppError::NotFound(_))
        ));

        // PDFs follow the read rules.
        assert!(
            recipes
                .pdf(None, &private.id, PdfTemplate::DefaultEnglish)
                .await
                .is_err()
        );
        assert!(
            recipes
                .pdf(Some(&ana), &private.id, PdfTemplate::DefaultEnglish)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn guests_can_be_disabled() {
        let store = Arc::new(InMemory::default());
        let recipes = use_cases(&store, false);
        let shared = recipes.create(&identity("ana"), draft(true)).await.unwrap();
        assert!(matches!(
            recipes.get(None, &shared.id).await,
            Err(AppError::Unauthorized(_))
        ));
        assert!(
            recipes
                .list_shared(None, PageQuery::default())
                .await
                .is_err()
        );
        assert!(
            recipes
                .list_shared(Some(&identity("bob")), PageQuery::default())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn rejects_foreign_categories_and_invalid_drafts() {
        let store = Arc::new(InMemory::default());
        let recipes = use_cases(&store, true);
        let mut d = draft(true);
        d.category_id = Some("someone-elses".into());
        assert!(
            matches!(recipes.create(&identity("ana"), d).await, Err(AppError::Validation(m)) if m.contains("Category"))
        );

        let mut d = draft(true);
        d.steps.clear();
        assert!(matches!(
            recipes.create(&identity("ana"), d).await,
            Err(AppError::Validation(_))
        ));
    }
}
