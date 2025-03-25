// titles/handlers.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::titles::models::Title;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use chrono::NaiveDate;

// Create Title
pub async fn create_title_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_title): Json<Title>,
) -> Result<Json<String>, StatusCode> {
    let query = "INSERT INTO titles (emp_no, title, from_date, to_date) VALUES (?, ?, ?, ?)";
    let result = sqlx::query(query)
        .bind(new_title.emp_no)
        .bind(&new_title.title)
        .bind(&new_title.from_date)
        .bind(new_title.to_date.as_ref()) // Handle Option<NaiveDate>
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(new_title).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All Titles
pub async fn title_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM titles";
    let titles = sqlx::query_as::<_, Title>(query)
        .fetch_all(&state.db)
        .await;

    match titles {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Title by emp_no, title, and from_date
pub async fn get_title_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM titles WHERE emp_no = ? AND title = ? AND from_date = ?";
    let title = sqlx::query_as::<_, Title>(query)
        .bind(emp_no)
        .bind(title)
        .bind(from_date)
        .fetch_one(&state.db)
        .await;

    match title {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Title
pub async fn edit_title_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>,
    Json(updated_title): Json<Title>,
) -> Result<Json<String>, StatusCode> {
    let query = "UPDATE titles SET to_date = ? WHERE emp_no = ? AND title = ? AND from_date = ?";
    let result = sqlx::query(query)
        .bind(updated_title.to_date.as_ref()) // Handle Option<NaiveDate>
        .bind(emp_no)
        .bind(title)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_title).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_title_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>,
) -> StatusCode {
    let query = "DELETE FROM titles WHERE emp_no = ? AND title = ? AND from_date = ?";
    let result = sqlx::query(query)
        .bind(emp_no)
        .bind(title)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}