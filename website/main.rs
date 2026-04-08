//
// ===THIS CODE WAS MODIFIED FROM MY MAIN MACHINE AS TO NOT LEAK ANY SENSITIVE INFORMATION===
//this only serves as proof of my work

use std::env;
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Form,
    Router,
};
use http::{HeaderValue, header::{CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS, X_CONTENT_TYPE_OPTIONS}};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::{
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    services::ServeDir,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::SmartIpKeyExtractor};
use axum_client_ip::InsecureClientIp; 
use validator::Validate;

#[derive(Deserialize, Validate)]
struct ContactForm {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 5000))]
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    fields: Option<serde_json::Value>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 1. db
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:app.db".into());
    let pool = SqlitePool::connect(&db_url).await.expect("DB Connection Failed");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            message TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    // 2. headers
    let csp = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; frame-ancestors 'none'; form-action 'self'";
    let csp_header = SetResponseHeaderLayer::overriding(CONTENT_SECURITY_POLICY, HeaderValue::from_str(csp).unwrap());
    let x_frame = SetResponseHeaderLayer::overriding(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    let x_content_type = SetResponseHeaderLayer::overriding(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));

    // 3. rate limiter (1 request per 60s)
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(60)
            .burst_size(1)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap(),
    );

    // 4. router
    let app = Router::new()
        .route("/api/contact", post(handle_contact))
        .with_state(pool)
        .layer(GovernorLayer { config: governor_conf })
        .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024)) // max 1MB payload (snti DOS)
        .layer(csp_header)
        .layer(x_frame)
        .layer(x_content_type)
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("public"));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3001));
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}

async fn handle_contact(
    State(pool): State<SqlitePool>,
    InsecureClientIp(ip): InsecureClientIp,
    Form(payload): Form<ContactForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    
    // input validation
    if let Err(_) = payload.validate() {
        return Err(ErrorResponse { error: "Invalid Input".into(), fields: None });
    }

    // injection protected (using .bind)
    let _ = sqlx::query("INSERT INTO messages (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&pool)
        .await;

    Ok((StatusCode::OK, Json(serde_json::json!({ "status": "sent" }))))
}
