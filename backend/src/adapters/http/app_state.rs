use std::sync::Arc;

use axum::extract::FromRef;

use super::rate_limit::RateLimiter;
use crate::{
    infra::config::AppConfig,
    use_cases::{
        auth::AuthUseCases, recipe::RecipeUseCases, recipe_import::RecipeImportUseCases,
        system::SystemUseCases, taxonomy::TaxonomyUseCases, user::UserUseCases,
    },
};

/// Everything handlers need. Each handler extracts only the use cases it uses
/// (`State<Arc<RecipeUseCases>>`, ...) thanks to the `FromRef` impls below.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub auth_use_cases: Arc<AuthUseCases>,
    pub user_use_cases: Arc<UserUseCases>,
    pub taxonomy_use_cases: Arc<TaxonomyUseCases>,
    pub recipe_use_cases: Arc<RecipeUseCases>,
    pub import_use_cases: Arc<RecipeImportUseCases>,
    pub system_use_cases: Arc<SystemUseCases>,
    pub rate_limiter: Arc<RateLimiter>,
}

macro_rules! from_ref {
    ($($field:ident: $ty:ty),* $(,)?) => {$(
        impl FromRef<AppState> for Arc<$ty> {
            fn from_ref(app_state: &AppState) -> Self {
                app_state.$field.clone()
            }
        }
    )*};
}

from_ref! {
    config: AppConfig,
    auth_use_cases: AuthUseCases,
    user_use_cases: UserUseCases,
    taxonomy_use_cases: TaxonomyUseCases,
    recipe_use_cases: RecipeUseCases,
    import_use_cases: RecipeImportUseCases,
    system_use_cases: SystemUseCases,
}
