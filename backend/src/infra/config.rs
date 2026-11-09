//! Application configuration, read once at start-up from environment variables.
//!
//! Variable names match the previous backend so existing `docker-compose` files keep working.

use std::{env, net::SocketAddr, path::PathBuf, str::FromStr, time::Duration};

const DEV_JWT_SECRET: &str = "local-dev-secret-change-this-please-32chars";

#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Address the HTTP server binds to (`BIND_ADDR`, default `0.0.0.0:8080`).
    pub bind_addr: SocketAddr,
    /// Path of the SQLite database file (`DB_URL`).
    pub database_path: PathBuf,
    /// Maximum number of pooled SQLite connections (`DB_MAX_CONNECTIONS`).
    pub database_max_connections: u32,

    pub auth: AuthConfig,

    /// Origin allowed to call the API with credentials (`CORS_ALLOWED_ORIGIN`).
    pub cors_allowed_origin: String,
    /// Honour `X-Forwarded-For` for client IPs (`TRUST_PROXY`). Only enable behind a proxy you control.
    pub trust_proxy: bool,
    /// Allow unauthenticated visitors to browse shared recipes (`GUEST_ACCESS`).
    pub guest_access: bool,

    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// HMAC secret used to sign access tokens (`JWT_SECRET`).
    pub jwt_secret: String,
    /// Access-token lifetime (`JWT_EXPIRATION_SECONDS`).
    pub access_token_ttl: Duration,
    /// Refresh-token lifetime (`REFRESH_TOKEN_DAYS`).
    pub refresh_token_ttl: Duration,
    /// Mark cookies `Secure` (`COOKIE_SECURE`). Requires HTTPS.
    pub cookie_secure: bool,
    /// Consecutive failed logins before an account is locked (`MAX_LOGIN_ATTEMPTS`).
    pub max_login_attempts: i64,
    /// How long a locked account stays locked (`LOCKOUT_DURATION_MINUTES`).
    pub lockout_duration: Duration,
    /// Login/register requests allowed per client IP per minute (`AUTH_RATE_LIMIT_PER_MINUTE`).
    pub rate_limit_per_minute: u32,
    /// bcrypt work factor for new hashes (`BCRYPT_COST`).
    pub bcrypt_cost: u32,
}

#[derive(Debug, Clone)]
pub struct OllamaConfig {
    /// Base URL of the Ollama server (`OLLAMA_URL`).
    pub url: String,
    /// Model used to extract recipes (`OLLAMA_MODEL`).
    pub model: String,
    /// Maximum time a single generation may take (`OLLAMA_TIMEOUT_SECONDS`).
    pub timeout: Duration,
    /// Maximum concurrent LLM requests (`LLM_MAX_CONCURRENCY`); protects memory and the model.
    pub max_concurrency: usize,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid value for environment variable {name}: {reason}")]
pub struct ConfigError {
    name: &'static str,
    reason: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => {
                tracing::warn!("JWT_SECRET is not set; using an insecure development secret");
                DEV_JWT_SECRET.to_owned()
            }
        };
        if jwt_secret.len() < 32 {
            tracing::warn!("JWT_SECRET is shorter than 32 bytes; use a longer random value");
        }

        Ok(Self {
            bind_addr: parse("BIND_ADDR", "0.0.0.0:8080")?,
            database_path: database_path(&var_or("DB_URL", "cocookies.db")),
            database_max_connections: parse("DB_MAX_CONNECTIONS", "4")?,
            auth: AuthConfig {
                jwt_secret,
                access_token_ttl: Duration::from_secs(parse("JWT_EXPIRATION_SECONDS", "10800")?),
                refresh_token_ttl: Duration::from_secs(
                    parse::<u64>("REFRESH_TOKEN_DAYS", "7")? * 24 * 3600,
                ),
                cookie_secure: parse_bool("COOKIE_SECURE", false)?,
                max_login_attempts: parse("MAX_LOGIN_ATTEMPTS", "5")?,
                lockout_duration: Duration::from_secs(
                    parse::<u64>("LOCKOUT_DURATION_MINUTES", "15")? * 60,
                ),
                rate_limit_per_minute: parse("AUTH_RATE_LIMIT_PER_MINUTE", "5")?,
                bcrypt_cost: parse("BCRYPT_COST", "10")?,
            },
            cors_allowed_origin: var_or("CORS_ALLOWED_ORIGIN", "http://localhost:5173"),
            trust_proxy: parse_bool("TRUST_PROXY", false)?,
            guest_access: parse_bool("GUEST_ACCESS", true)?,
            ollama: OllamaConfig {
                url: var_or("OLLAMA_URL", "http://localhost:11434")
                    .trim_end_matches('/')
                    .to_owned(),
                model: var_or("OLLAMA_MODEL", "gemma3:4b"),
                timeout: Duration::from_secs(parse("OLLAMA_TIMEOUT_SECONDS", "300")?),
                max_concurrency: parse("LLM_MAX_CONCURRENCY", "2")?,
            },
        })
    }

    /// A configuration suitable for tests: in-memory friendly defaults, no env access.
    pub fn for_tests(database_path: PathBuf) -> Self {
        Self {
            bind_addr: "127.0.0.1:0".parse().unwrap(),
            database_path,
            database_max_connections: 2,
            auth: AuthConfig {
                jwt_secret: DEV_JWT_SECRET.to_owned(),
                access_token_ttl: Duration::from_secs(3600),
                refresh_token_ttl: Duration::from_secs(7 * 24 * 3600),
                cookie_secure: false,
                max_login_attempts: 5,
                lockout_duration: Duration::from_secs(15 * 60),
                rate_limit_per_minute: 1000,
                bcrypt_cost: 4,
            },
            cors_allowed_origin: "http://localhost:5173".to_owned(),
            trust_proxy: false,
            guest_access: true,
            ollama: OllamaConfig {
                url: "http://127.0.0.1:9".to_owned(),
                model: "test".to_owned(),
                timeout: Duration::from_secs(1),
                max_concurrency: 1,
            },
        }
    }
}

/// Accepts plain paths as well as the JDBC (`jdbc:sqlite:/app/data/x.db`) and sqlx
/// (`sqlite://x.db`) URL styles, so old deployments can be upgraded without touching their env.
fn database_path(raw: &str) -> PathBuf {
    let s = raw.trim();
    let s = s.strip_prefix("jdbc:").unwrap_or(s);
    let s = s.strip_prefix("sqlite:").unwrap_or(s);
    let s = s.strip_prefix("//").unwrap_or(s);
    let s = s.split('?').next().unwrap_or(s);
    PathBuf::from(s)
}

fn var_or(name: &str, default: &str) -> String {
    env::var(name)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default.to_owned())
}

fn parse<T: FromStr>(name: &'static str, default: &str) -> Result<T, ConfigError>
where
    T::Err: std::fmt::Display,
{
    var_or(name, default)
        .trim()
        .parse()
        .map_err(|e: T::Err| ConfigError {
            name,
            reason: e.to_string(),
        })
}

fn parse_bool(name: &'static str, default: bool) -> Result<bool, ConfigError> {
    match env::var(name)
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Err(_) | Ok("") => Ok(default),
        Ok("1" | "true" | "yes" | "on") => Ok(true),
        Ok("0" | "false" | "no" | "off") => Ok(false),
        Ok(other) => Err(ConfigError {
            name,
            reason: format!("expected a boolean, got '{other}'"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::database_path;
    use std::path::PathBuf;

    #[test]
    fn database_path_accepts_legacy_urls() {
        assert_eq!(
            database_path("jdbc:sqlite:/app/data/c.db"),
            PathBuf::from("/app/data/c.db")
        );
        assert_eq!(
            database_path("sqlite://c.db?mode=rwc"),
            PathBuf::from("c.db")
        );
        assert_eq!(database_path("data/c.db"), PathBuf::from("data/c.db"));
    }
}
