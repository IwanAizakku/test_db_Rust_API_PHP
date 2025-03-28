// salaries/handlers.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::salaries::models::Salary;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;

// Create Salary
pub async fn create_salary_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_salary): Json<Salary>,
) -> Result<Json<String>, StatusCode> {
    let query = "INSERT INTO salaries (emp_no, salary, from_date, to_date) VALUES (?, ?, ?, ?)";
    let result = sqlx::query(query)
        .bind(new_salary.emp_no)
        .bind(new_salary.salary)
        .bind(&new_salary.from_date)
        .bind(&new_salary.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(new_salary).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All Salaries
pub async fn salary_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM salaries LIMIT 10";
    let salaries = sqlx::query_as::<_, Salary>(query)
        .fetch_all(&state.db)
        .await;

    match salaries {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Salary by emp_no and from_date
pub async fn get_salary_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, from_date)): Path<(i32, chrono::NaiveDate)>,
) -> Result<Json<String>, StatusCode> {
    let query = "SELECT * FROM salaries WHERE emp_no = ? AND from_date = ?";
    let salary = sqlx::query_as::<_, Salary>(query)
        .bind(emp_no)
        .bind(from_date)
        .fetch_one(&state.db)
        .await;

    match salary {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Salary
pub async fn edit_salary_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, from_date)): Path<(i32, chrono::NaiveDate)>,
    Json(updated_salary): Json<Salary>,
) -> Result<Json<String>, StatusCode> {
    let query = "UPDATE salaries SET salary = ?, to_date = ? WHERE emp_no = ? AND from_date = ?";
    let result = sqlx::query(query)
        .bind(updated_salary.salary)
        .bind(&updated_salary.to_date)
        .bind(emp_no)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_salary).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_salary_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path((emp_no, from_date)): Path<(i32, chrono::NaiveDate)>,
) -> StatusCode {
    let query = "DELETE FROM salaries WHERE emp_no = ? AND from_date = ?";
    let result = sqlx::query(query)
        .bind(emp_no)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}