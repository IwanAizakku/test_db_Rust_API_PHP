// Departments functions
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;

use crate::db::AppState;
use crate::departments::models::Department;
use crate::sodium::sodium_crypto::{encrypt_json, get_key}; // Import get_key
use crate::auth::Claims;

// Create Department
pub async fn create_department_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_department): Json<Department>,
) -> Result<Json<String>, StatusCode> {
    let query = "INSERT INTO departments (dept_no, dept_name) VALUES (?, ?)";
    
    let result = sqlx::query(query)
        .bind(&new_department.dept_no)
        .bind(&new_department.dept_name)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(new_department).unwrap();
            let key = get_key(); // Retrieve the key
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All Departments
pub async fn department_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM departments LIMIT 10";
    let departments = sqlx::query_as::<_, Department>(query)
        .fetch_all(&state.db)
        .await;

    match departments {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key(); // Retrieve the key
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Department by ID
pub async fn get_department_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM departments WHERE dept_no = ?";
    let department = sqlx::query_as::<_, Department>(query)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match department {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key(); // Retrieve the key
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Department
pub async fn edit_department_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
    Json(updated_department): Json<Department>,
) -> Result<Json<String>, StatusCode> {
    let query = "UPDATE departments SET dept_name = ? WHERE dept_no = ?";
    let result = sqlx::query(query)
        .bind(&updated_department.dept_name)
        .bind(dept_no)
        .execute(&state.db)
        .await;
    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_department).unwrap();
            let key = get_key(); // Retrieve the key
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_department_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
) -> StatusCode {
    let query = "DELETE FROM departments WHERE dept_no = ?";
    let result = sqlx::query(query)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}