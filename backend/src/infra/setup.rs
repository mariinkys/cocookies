//! Composition root: builds the adapters, injects them into the use cases and assembles the
//! [`AppState`]. Swapping an implementation (another database, hasher, LLM...) only touches
//! this file.

use std::{sync::Arc, time::Duration};

use tracing_subscriber::EnvFilter;

use crate::{
    adapters::{
        crypto::{
            bcrypt::BcryptPasswordHasher, jwt::JwtAccessTokens, refresh_token::RandomRefreshTokens,
        },
        documents::TextDocumentReader,
        http::{app_state::AppState, rate_limit::RateLimiter},
        llm::OllamaRecipeExtractor,
        net::SafeFetcher,
        pdf::KrillaRecipeRenderer,
        persistence::SqlitePersistence,
    },
    infra::{config::AppConfig, db::init_db},
    use_cases::{
        auth::{AuthPolicy, AuthUseCases},
        recipe::{RecipePolicy, RecipeUseCases},
        recipe_import::RecipeImportUseCases,
        system::{ServerInfo, SystemUseCases},
        taxonomy::TaxonomyUseCases,
        user::UserUseCases,
    },
};

pub async fn init_app_state(config: AppConfig) -> anyhow::Result<AppState> {
    // reqwest is built without a bundled crypto backend; use ring. Ignore the error if a
    // provider was already installed (tests build several states).
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Adapters
    let pool = init_db(&config.database_path, config.database_max_connections).await?;
    let persistence = Arc::new(SqlitePersistence::new(pool));
    let hasher = Arc::new(BcryptPasswordHasher::new(config.auth.bcrypt_cost));
    let access_tokens = Arc::new(JwtAccessTokens::new(
        &config.auth.jwt_secret,
        config.auth.access_token_ttl,
    ));
    let refresh_tokens = Arc::new(RandomRefreshTokens);
    let fetcher = Arc::new(SafeFetcher::new(Duration::from_secs(20))?);
    let renderer = Arc::new(KrillaRecipeRenderer::new().map_err(anyhow::Error::msg)?);
    let extractor = Arc::new(OllamaRecipeExtractor::new(
        &config.ollama.url,
        &config.ollama.model,
        config.ollama.timeout,
        config.ollama.max_concurrency,
    )?);
    let documents = Arc::new(TextDocumentReader);

    // Use cases
    let auth_use_cases = AuthUseCases::new(
        persistence.clone(),
        persistence.clone(),
        hasher.clone(),
        access_tokens,
        refresh_tokens,
        AuthPolicy {
            max_login_attempts: config.auth.max_login_attempts,
            lockout_duration: config.auth.lockout_duration,
            refresh_token_ttl: config.auth.refresh_token_ttl,
        },
    );
    let user_use_cases = UserUseCases::new(persistence.clone(), hasher);
    let taxonomy_use_cases = TaxonomyUseCases::new(persistence.clone());
    let recipe_use_cases = RecipeUseCases::new(
        persistence.clone(),
        persistence.clone(),
        fetcher.clone(),
        renderer,
        RecipePolicy {
            guest_access: config.guest_access,
        },
    );
    let import_use_cases = RecipeImportUseCases::new(extractor, documents, fetcher);
    let system_use_cases = SystemUseCases::new(
        persistence,
        ServerInfo {
            version: env!("CARGO_PKG_VERSION"),
            guest_access: config.guest_access,
        },
    );

    Ok(AppState {
        rate_limiter: Arc::new(RateLimiter::per_minute(config.auth.rate_limit_per_minute)),
        config: Arc::new(config),
        auth_use_cases: Arc::new(auth_use_cases),
        user_use_cases: Arc::new(user_use_cases),
        taxonomy_use_cases: Arc::new(taxonomy_use_cases),
        recipe_use_cases: Arc::new(recipe_use_cases),
        import_use_cases: Arc::new(import_use_cases),
        system_use_cases: Arc::new(system_use_cases),
    })
}

/// Logging. Control verbosity with `RUST_LOG` (e.g. `RUST_LOG=debug,sqlx=info`).
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,tower_http=info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
