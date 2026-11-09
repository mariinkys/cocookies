//! The axum application: API routes plus global middleware.

use std::time::Duration;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, HeaderValue, Method, Request, header},
    response::IntoResponse,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::{
    adapters::{self, http::app_state::AppState},
    app_error::AppError,
};

/// Default request-body cap (recipes are small JSON documents).
const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;

pub fn create_app(app_state: AppState) -> Router {
    let config = app_state.config.clone();

    let mut router = Router::new()
        .nest("/api", adapters::http::routes::router(app_state.clone()))
        .fallback(|| async { AppError::NotFound("Resource not found".into()).into_response() })
        .with_state(app_state)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES));

    // Security headers (applied unless a handler set them explicitly).
    for (name, value) in [
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::X_FRAME_OPTIONS, "DENY"),
        (header::REFERRER_POLICY, "no-referrer"),
        (header::CACHE_CONTROL, "no-store"),
    ] {
        router = router.layer(SetResponseHeaderLayer::if_not_present(
            name,
            HeaderValue::from_static(value),
        ));
    }
    if config.auth.cookie_secure {
        router = router.layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ));
    }

    router.layer(cors(&config.cors_allowed_origin)).layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            tracing::debug_span!("http-request", method = %request.method(), uri = %request.uri())
        }),
    )
}

fn cors(origin: &str) -> CorsLayer {
    let origin = match HeaderValue::from_str(origin) {
        Ok(v) => AllowOrigin::exact(v),
        Err(_) => {
            tracing::warn!(
                origin,
                "invalid CORS_ALLOWED_ORIGIN; cross-origin requests disabled"
            );
            AllowOrigin::list(Vec::<HeaderValue>::new())
        }
    };
    CorsLayer::new()
        .allow_origin(origin)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE])
        .expose_headers([HeaderName::from_static("content-disposition")])
        .max_age(Duration::from_secs(3600))
}
