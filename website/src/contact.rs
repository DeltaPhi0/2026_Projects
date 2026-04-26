use axum::http::StatusCode;
use axum::{extract::State, Form, Json};
use axum_client_ip::InsecureClientIp;
use sqlx::SqlitePool;
use validator::Validate;

use crate::errors::ErrorResponse;
use crate::models::{ContactForm, SuccessResponse};

pub async fn handle_contact(
    State(pool): State<SqlitePool>,
    InsecureClientIp(ip): InsecureClientIp,
    Form(payload): Form<ContactForm>,
) -> Result<(StatusCode, Json<SuccessResponse>), ErrorResponse> {
    payload.validate().map_err(|e| ErrorResponse {
        error: "Invalid input".into(),
        fields: Some(serde_json::to_value(e.field_errors()).unwrap()),
    })?;
    if payload.name.trim().is_empty()
        || payload.email.trim().is_empty()
        || payload.message.trim().is_empty()
    {
        return Err(ErrorResponse {
            error: "All fields are required".into(),
            fields: None,
        });
    }

    sqlx::query("INSERT INTO messages (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&pool)
        .await
        .map_err(|_| ErrorResponse {
            error: "Database error".into(),
            fields: None,
        })?;

    Ok((
        StatusCode::OK,
        Json(SuccessResponse {
            status: "success",
            message: "Message committed to the Crimson Ledger",
        }),
    ))
}

