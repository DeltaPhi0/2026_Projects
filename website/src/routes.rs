use axum::{
    http::{
        header::{CONTENT_SECURITY_POLICY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS},
        HeaderValue,
    },
    routing::post,
    Router,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::{
    limit::RequestBodyLimitLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
};

use crate::cli::{handle_autocomplete, handle_execute};
use crate::contact::handle_contact;

pub fn create_router(pool: SqlitePool) -> Router {

    let contact_governor = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(60)
            .burst_size(2)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let cli_governor = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(15)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let csp = "default-src 'self'; script-src 'self' https://cdnjs.cloudflare.com; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'";

    let contact_routes = Router::new()
        .route("/èath/to/api", post(handle_contact))
        .layer(GovernorLayer {
            config: contact_governor,
        });

    let cli_routes = Router::new()
        .route("/path/to/api", post(handle_execute))
        .route("/path/to/api", post(handle_autocomplete))
        .layer(GovernorLayer {
            config: cli_governor,
        });

    Router::new()
        .merge(contact_routes)
        .merge(cli_routes)
        .with_state(pool)
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
        .layer(SetResponseHeaderLayer::overriding(
            CONTENT_SECURITY_POLICY,
            HeaderValue::from_str(csp).unwrap(),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .fallback_service(ServeDir::new("public"))
}
