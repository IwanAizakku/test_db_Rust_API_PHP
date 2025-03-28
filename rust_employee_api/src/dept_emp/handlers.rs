// dept_emp/handlers.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_emp::models::DeptEmp;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;

// Create DeptEmp
pub async fn create_dept_emp_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_dept_emp): Json<DeptEmp>,
) -> Result<Json<String>, StatusCode> {
    let query = "INSERT INTO dept_emp (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";
    let result = sqlx::query(query)
        .bind(new_dept_emp.emp_no)
        .bind(&new_dept_emp.dept_no)
        .bind(&new_dept_emp.from_date)
        .bind(&new_dept_emp.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(new_dept_emp).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All DeptEmps
pub async fn dept_emp_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM dept_emp LIMIT 10";
    let dept_emps = sqlx::query_as::<_, DeptEmp>(query)
        .fetch_all(&state.db)
        .await;

    match dept_emps {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get DeptEmp by emp_no and dept_no
pub async fn get_dept_emp_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM dept_emp WHERE emp_no = ? AND dept_no = ?";
    let dept_emp = sqlx::query_as::<_, DeptEmp>(query)
        .bind(emp_no)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match dept_emp {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update DeptEmp
pub async fn edit_dept_emp_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
    Json(updated_dept_emp): Json<DeptEmp>,
) -> Result<Json<String>, StatusCode> {
    let query = "UPDATE dept_emp SET from_date = ?, to_date = ? WHERE emp_no = ? AND dept_no = ?";
    let result = sqlx::query(query)
        .bind(&updated_dept_emp.from_date)
        .bind(&updated_dept_emp.to_date)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_dept_emp).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_dept_emp_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, dept_no)): Path<(i32, String)>,
) -> StatusCode {
    let query = "DELETE FROM dept_emp WHERE emp_no = ? AND dept_no = ?";
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