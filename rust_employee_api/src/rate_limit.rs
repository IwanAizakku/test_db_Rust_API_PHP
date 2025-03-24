// rate_limit.rs
use axum::{
    extract::State,
    http::StatusCode,
    Json,
    middleware::Next,
};
use serde_json::{json, Value};
use sqlx::{query, query_as};
use std::sync::Arc;

use crate::db::{AppState, RateLimit};
use crate::auth::Claims;

pub async fn rate_limit_middleware( 
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    request: axum::http::Request<axum::body::Body>, 
    next: Next, 
) -> Result<axum::response::Response, (StatusCode, Json<Value>)> {
    let username = claims.sub;

    let rate_limit_result = query_rate_limit(&app_state, &username).await;

    match rate_limit_result {
        Ok(rate_limit) => {
            if rate_limit.remaining_requests <= 0 {
                return Err((StatusCode::TOO_MANY_REQUESTS, Json(json!({"error": "Rate limit exceeded"}))));
            }

            update_rate_limit(&app_state, &username, rate_limit.remaining_requests - 1).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal server error"}))))?;

            Ok(next.run(request).await)
        }
        Err(_) => {
            create_new_rate_limit(&app_state, &username).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal server error"}))))?;
            Ok(next.run(request).await)
        }
    }
}

async fn query_rate_limit(app_state: &AppState, username: &str) -> Result<RateLimit, sqlx::Error> {
    query_as!(
        RateLimit,
        "SELECT username, remaining_requests FROM rate_limits WHERE username = ?",
        username
    )
    .fetch_one(&app_state.db)
    .await
}

async fn update_rate_limit(app_state: &AppState, username: &str, remaining_requests: i32) -> Result<(), sqlx::Error> {
    query!(
        "UPDATE rate_limits SET remaining_requests = ? WHERE username = ?",
        remaining_requests,
        username
    )
    .execute(&app_state.db)
    .await
    .map(|_| ())
}

async fn create_new_rate_limit(app_state: &AppState, username: &str) -> Result<(), sqlx::Error>{
    let initial_limit = 10; // Set request limit
    query!(
        "INSERT INTO rate_limits (username, remaining_requests) VALUES (?, ?)",
        username,
        initial_limit
    ).execute(&app_state.db).await.map(|_| ())
}