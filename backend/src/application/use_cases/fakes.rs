//! In-memory implementations of the ports, for unit-testing use cases without a database,
//! crypto, network or PDF engine.

use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};

use async_trait::async_trait;

use super::{
    auth::{AccessTokenService, RefreshTokenFactory, SessionPersistence},
    recipe::{RecipePersistence, RecipeRenderer, RemoteFetcher, RemoteResource},
    taxonomy::TaxonomyPersistence,
    user::{PasswordHasher, UserPersistence},
};
use crate::{
    app_error::{AppError, AppResult},
    entities::{
        page::PageQuery,
        recipe::{
            Ingredient, PdfTemplate, Recipe, RecipeDraft, RecipeScope, RecipeSort, RecipeSummary,
            Step,
        },
        session::RefreshToken,
        taxonomy::{TaxonomyDraft, TaxonomyItem, TaxonomyKind, TaxonomySort},
        user::User,
    },
};

#[derive(Default)]
struct State {
    users: Vec<User>,
    tokens: Vec<(String, RefreshToken)>,
    taxonomy: Vec<(TaxonomyKind, TaxonomyItem)>,
    recipes: Vec<(String, String, RecipeDraft)>, // (id, owner, draft)
}

/// One in-memory "database" implementing every persistence port.
#[derive(Default)]
pub struct InMemory {
    state: Mutex<State>,
    ids: AtomicU64,
}

impl InMemory {
    fn next_id(&self) -> String {
        format!("id-{}", self.ids.fetch_add(1, Ordering::Relaxed))
    }

    fn with<R>(&self, f: impl FnOnce(&mut State) -> R) -> R {
        f(&mut self.state.lock().unwrap())
    }

    pub fn user_by_email(&self, email: &str) -> Option<User> {
        self.with(|s| s.users.iter().find(|u| u.email == email).cloned())
    }

    /// Moves every account lock into the past.
    pub fn expire_locks(&self) {
        self.with(|s| {
            for u in &mut s.users {
                if u.locked_until.is_some() {
                    u.locked_until = Some("1970-01-01T00:00:00Z".into());
                }
            }
        });
    }
}

#[async_trait]
impl UserPersistence for InMemory {
    async fn find_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        Ok(self.with(|s| s.users.iter().find(|u| u.id == id).cloned()))
    }
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        Ok(self.user_by_email(email))
    }
    async fn user_email_taken(&self, email: &str, except_id: Option<&str>) -> AppResult<bool> {
        Ok(self.with(|s| {
            s.users
                .iter()
                .any(|u| u.email == email && Some(u.id.as_str()) != except_id)
        }))
    }
    async fn create_user(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User> {
        let user = User {
            id: self.next_id(),
            name: name.into(),
            email: email.into(),
            password_hash: password_hash.into(),
            failed_login_attempts: 0,
            locked_until: None,
            created_at: crate::time::now_iso(),
            updated_at: crate::time::now_iso(),
        };
        self.with(|s| s.users.push(user.clone()));
        Ok(user)
    }
    async fn update_user_profile(&self, id: &str, name: &str, email: &str) -> AppResult<()> {
        self.with(|s| {
            if let Some(u) = s.users.iter_mut().find(|u| u.id == id) {
                u.name = name.into();
                u.email = email.into();
            }
        });
        Ok(())
    }
    async fn update_user_password(&self, id: &str, password_hash: &str) -> AppResult<()> {
        self.with(|s| {
            if let Some(u) = s.users.iter_mut().find(|u| u.id == id) {
                u.password_hash = password_hash.into();
            }
        });
        Ok(())
    }
    async fn record_failed_login(
        &self,
        id: &str,
        attempts: i64,
        locked_until: Option<&str>,
    ) -> AppResult<()> {
        self.with(|s| {
            if let Some(u) = s.users.iter_mut().find(|u| u.id == id) {
                u.failed_login_attempts = attempts;
                u.locked_until = locked_until.map(Into::into);
            }
        });
        Ok(())
    }
    async fn reset_failed_logins(&self, id: &str) -> AppResult<()> {
        self.record_failed_login(id, 0, None).await
    }
    async fn delete_user(&self, id: &str) -> AppResult<bool> {
        Ok(self.with(|s| {
            let before = s.users.len();
            s.users.retain(|u| u.id != id);
            before != s.users.len()
        }))
    }
}

#[async_trait]
impl SessionPersistence for InMemory {
    async fn store_refresh_token(
        &self,
        user_id: &str,
        fingerprint: &str,
        expires_at: &str,
    ) -> AppResult<()> {
        let token = RefreshToken {
            id: self.next_id(),
            user_id: user_id.into(),
            expires_at: expires_at.into(),
            revoked: false,
        };
        self.with(|s| s.tokens.push((fingerprint.into(), token)));
        Ok(())
    }
    async fn find_refresh_token(&self, fingerprint: &str) -> AppResult<Option<RefreshToken>> {
        Ok(self.with(|s| {
            s.tokens
                .iter()
                .find(|(f, _)| f == fingerprint)
                .map(|(_, t)| t.clone())
        }))
    }
    async fn consume_refresh_token(&self, id: &str) -> AppResult<bool> {
        Ok(self.with(
            |s| match s.tokens.iter_mut().find(|(_, t)| t.id == id && !t.revoked) {
                Some((_, t)) => {
                    t.revoked = true;
                    true
                }
                None => false,
            },
        ))
    }
    async fn revoke_refresh_token(&self, fingerprint: &str) -> AppResult<()> {
        self.with(|s| {
            s.tokens
                .iter_mut()
                .filter(|(f, _)| f == fingerprint)
                .for_each(|(_, t)| t.revoked = true)
        });
        Ok(())
    }
    async fn revoke_user_refresh_tokens(&self, user_id: &str) -> AppResult<()> {
        self.with(|s| {
            s.tokens
                .iter_mut()
                .filter(|(_, t)| t.user_id == user_id)
                .for_each(|(_, t)| t.revoked = true)
        });
        Ok(())
    }
    async fn delete_refresh_tokens_expired_before(&self, instant: &str) -> AppResult<u64> {
        Ok(self.with(|s| {
            let before = s.tokens.len();
            s.tokens.retain(|(_, t)| t.expires_at.as_str() >= instant);
            (before - s.tokens.len()) as u64
        }))
    }
}

#[async_trait]
impl TaxonomyPersistence for InMemory {
    async fn page_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        _query: &PageQuery<TaxonomySort>,
    ) -> AppResult<(Vec<TaxonomyItem>, i64)> {
        let items = self.list_taxonomy(kind, user_id).await?;
        let total = items.len() as i64;
        Ok((items, total))
    }
    async fn list_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
    ) -> AppResult<Vec<TaxonomyItem>> {
        Ok(self.with(|s| {
            s.taxonomy
                .iter()
                .filter(|(k, i)| *k == kind && i.user_id == user_id)
                .map(|(_, i)| i.clone())
                .collect()
        }))
    }
    async fn find_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<Option<TaxonomyItem>> {
        Ok(self.with(|s| {
            s.taxonomy
                .iter()
                .find(|(k, i)| *k == kind && i.id == id)
                .map(|(_, i)| i.clone())
        }))
    }
    async fn taxonomy_name_taken(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        name: &str,
        except_id: Option<&str>,
    ) -> AppResult<bool> {
        Ok(self.with(|s| {
            s.taxonomy.iter().any(|(k, i)| {
                *k == kind
                    && i.user_id == user_id
                    && i.name == name
                    && Some(i.id.as_str()) != except_id
            })
        }))
    }
    async fn create_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<TaxonomyItem> {
        let item = TaxonomyItem {
            id: self.next_id(),
            user_id: user_id.into(),
            name: draft.name.clone(),
            description: draft.description.clone(),
            created_at: String::new(),
            updated_at: String::new(),
        };
        self.with(|s| s.taxonomy.push((kind, item.clone())));
        Ok(item)
    }
    async fn update_taxonomy(
        &self,
        kind: TaxonomyKind,
        id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<()> {
        self.with(|s| {
            if let Some((_, i)) = s
                .taxonomy
                .iter_mut()
                .find(|(k, i)| *k == kind && i.id == id)
            {
                i.name = draft.name.clone();
                i.description = draft.description.clone();
            }
        });
        Ok(())
    }
    async fn delete_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<()> {
        self.with(|s| s.taxonomy.retain(|(k, i)| !(*k == kind && i.id == id)));
        Ok(())
    }
    async fn taxonomy_owned_by(
        &self,
        kind: TaxonomyKind,
        id: &str,
        user_id: &str,
    ) -> AppResult<bool> {
        Ok(self
            .find_taxonomy(kind, id)
            .await?
            .is_some_and(|i| i.user_id == user_id))
    }
}

fn to_recipe(id: &str, owner: &str, d: &RecipeDraft) -> Recipe {
    Recipe {
        id: id.into(),
        user_id: owner.into(),
        category: None,
        difficulty: None,
        title: d.title.clone(),
        description: d.description.clone(),
        prep_time: d.prep_time,
        cook_time: d.cook_time,
        servings: d.servings,
        image_url: d.image_url.clone(),
        shared: d.shared,
        steps: d
            .steps
            .iter()
            .map(|s| Step {
                id: 0,
                step_number: s.step_number,
                instructions: s.instructions.clone(),
                duration: s.duration,
            })
            .collect(),
        ingredients: d
            .ingredients
            .iter()
            .map(|i| Ingredient {
                id: 0,
                name: i.name.clone(),
                quantity: i.quantity,
                unit: i.unit.clone(),
                notes: i.notes.clone(),
                sort_order: i.sort_order,
            })
            .collect(),
        nutrition: None,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

#[async_trait]
impl RecipePersistence for InMemory {
    async fn page_recipes(
        &self,
        scope: &RecipeScope,
        _query: &PageQuery<RecipeSort>,
    ) -> AppResult<(Vec<RecipeSummary>, i64)> {
        let items: Vec<RecipeSummary> = self.with(|s| {
            s.recipes
                .iter()
                .filter(|(_, owner, d)| match scope {
                    RecipeScope::OwnedBy(user) => owner == user,
                    RecipeScope::Shared => d.shared,
                })
                .map(|(id, owner, d)| RecipeSummary {
                    id: id.clone(),
                    user_id: owner.clone(),
                    category: None,
                    difficulty: None,
                    title: d.title.clone(),
                    description: d.description.clone(),
                    prep_time: d.prep_time,
                    cook_time: d.cook_time,
                    servings: d.servings,
                    image_url: d.image_url.clone(),
                    shared: d.shared,
                    created_at: String::new(),
                    updated_at: String::new(),
                })
                .collect()
        });
        let total = items.len() as i64;
        Ok((items, total))
    }
    async fn find_recipe(&self, id: &str) -> AppResult<Option<Recipe>> {
        Ok(self.with(|s| {
            s.recipes
                .iter()
                .find(|(i, _, _)| i == id)
                .map(|(i, o, d)| to_recipe(i, o, d))
        }))
    }
    async fn recipe_owner(&self, id: &str) -> AppResult<Option<String>> {
        Ok(self.with(|s| {
            s.recipes
                .iter()
                .find(|(i, _, _)| i == id)
                .map(|(_, o, _)| o.clone())
        }))
    }
    async fn create_recipe(&self, user_id: &str, draft: &RecipeDraft) -> AppResult<String> {
        let id = self.next_id();
        self.with(|s| s.recipes.push((id.clone(), user_id.into(), draft.clone())));
        Ok(id)
    }
    async fn update_recipe(&self, id: &str, draft: &RecipeDraft) -> AppResult<()> {
        self.with(|s| {
            if let Some((_, _, d)) = s.recipes.iter_mut().find(|(i, _, _)| i == id) {
                *d = draft.clone();
            }
        });
        Ok(())
    }
    async fn delete_recipe(&self, id: &str) -> AppResult<()> {
        self.with(|s| s.recipes.retain(|(i, _, _)| i != id));
        Ok(())
    }
}

/// "Hashes" by prefixing; good enough to check the use cases call the hasher.
pub struct FakeHasher;

#[async_trait]
impl PasswordHasher for FakeHasher {
    async fn hash_password(&self, password: &str) -> AppResult<String> {
        Ok(format!("hashed:{password}"))
    }
    async fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        Ok(hash == format!("hashed:{password}"))
    }
    async fn simulate_verification(&self, _password: &str) -> AppResult<()> {
        Ok(())
    }
}

/// Predictable tokens: `access:<user id>` and `refresh-<n>`.
#[derive(Default)]
pub struct FakeTokens {
    counter: AtomicU64,
}

impl AccessTokenService for FakeTokens {
    fn issue_access_token(&self, user_id: &str) -> AppResult<String> {
        Ok(format!("access:{user_id}"))
    }
    fn verify_access_token(&self, token: &str) -> Option<String> {
        token.strip_prefix("access:").map(Into::into)
    }
}

impl RefreshTokenFactory for FakeTokens {
    fn generate_refresh_token(&self) -> AppResult<String> {
        Ok(format!(
            "refresh-{}",
            self.counter.fetch_add(1, Ordering::Relaxed)
        ))
    }
    fn fingerprint(&self, token: &str) -> String {
        format!("fp:{token}")
    }
}

/// Serves `<p>fetched URL</p>`, or fails when the URL contains "fail".
pub struct FakeFetcher;

#[async_trait]
impl RemoteFetcher for FakeFetcher {
    async fn fetch(&self, url: &str, _max_bytes: usize) -> AppResult<RemoteResource> {
        if url.contains("fail") {
            return Err(AppError::Unprocessable("unreachable".into()));
        }
        Ok(RemoteResource {
            body: format!("<p>fetched {url}</p>").into_bytes(),
            content_type: None,
        })
    }
}

pub struct FakeRenderer;

#[async_trait]
impl RecipeRenderer for FakeRenderer {
    async fn render_pdf(
        &self,
        _: Recipe,
        _: PdfTemplate,
        _: Option<Vec<u8>>,
    ) -> AppResult<Vec<u8>> {
        Ok(b"%PDF-fake".to_vec())
    }
}
