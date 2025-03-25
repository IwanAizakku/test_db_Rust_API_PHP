// dept_manager/handlers.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_manager::models::DeptManager;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;

// Create DeptManager
pub async fn create_dept_manager_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_dept_manager): Json<DeptManager>,
) -> Result<Json<String>, StatusCode> {
    let query = "INSERT INTO dept_manager (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";
    let result = sqlx::query(query)
        .bind(new_dept_manager.emp_no)
        .bind(&new_dept_manager.dept_no)
        .bind(&new_dept_manager.from_date)
        .bind(&new_dept_manager.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(new_dept_manager).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All DeptManagers
pub async fn dept_manager_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM dept_manager";
    let dept_managers = sqlx::query_as::<_, DeptManager>(query)
        .fetch_all(&state.db)
        .await;

    match dept_managers {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get DeptManager by emp_no and dept_no
pub async fn get_dept_manager_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM dept_manager WHERE emp_no = ? AND dept_no = ?";
    let dept_manager = sqlx::query_as::<_, DeptManager>(query)
        .bind(emp_no)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match dept_manager {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update DeptManager
pub async fn edit_dept_manager_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
    Json(updated_dept_manager): Json<DeptManager>,
) -> Result<Json<String>, StatusCode> {
    let query = "UPDATE dept_manager SET from_date = ?, to_date = ? WHERE emp_no = ? AND dept_no = ?";
    let result = sqlx::query(query)
        .bind(&updated_dept_manager.from_date)
        .bind(&updated_dept_manager.to_date)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_dept_manager).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_dept_manager_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
) -> StatusCode {
    let query = "DELETE FROM dept_manager WHERE emp_no = ? AND dept_no = ?";
    let result = sqlx::query(query)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}