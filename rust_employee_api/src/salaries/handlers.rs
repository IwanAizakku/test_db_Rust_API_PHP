// salaries/handlers.rs
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use serde_json::{Value, from_value, to_value};

use crate::db::AppState;
use crate::salaries::models::Salary;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use sqlx::{query, query_as};
use chrono::NaiveDate;

// CRUD operations using POST
pub async fn salary_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_salary(&state, payload["salary"].clone()).await,
        "read_all" => read_all_salaries(&state).await,
        "read" => read_salary(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?).await,
        "update" => update_salary(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?, payload["salary"].clone()).await,
        "delete" => delete_salary(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_salary(state: &Arc<AppState>, salary_value: Value) -> Result<Json<String>, StatusCode> {
    let new_salary: Salary = from_value(salary_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO salaries (emp_no, salary, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = query(query_str)
        .bind(new_salary.emp_no)
        .bind(new_salary.salary)
        .bind(&new_salary.from_date)
        .bind(&new_salary.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_salary).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating salary: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_salaries(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM salaries LIMIT 10";

    let salaries = query_as::<_, Salary>(query_str)
        .fetch_all(&state.db)
        .await;

    match salaries {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching salaries: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_salary(state: &Arc<AppState>, emp_no: i32, from_date: NaiveDate) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM salaries WHERE emp_no = ? AND from_date = ?";
    let salary = query_as::<_, Salary>(query_str)
        .bind(emp_no)
        .bind(from_date)
        .fetch_one(&state.db)
        .await;

    match salary {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_salary(state: &Arc<AppState>, emp_no: i32, from_date: NaiveDate, salary_value: Value) -> Result<Json<String>, StatusCode> {
    let mut updated_salary: Salary = from_value(salary_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    //Use emp_no and from_date from path parameter
    updated_salary.emp_no = emp_no;
    updated_salary.from_date = from_date;

    let query_str = "UPDATE salaries SET salary = ?, to_date = ? WHERE emp_no = ? AND from_date = ?";

    let result = query(query_str)
        .bind(updated_salary.salary)
        .bind(&updated_salary.to_date)
        .bind(emp_no)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(updated_salary).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating salary: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_salary(state: &Arc<AppState>, emp_no: i32, from_date: NaiveDate) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM salaries WHERE emp_no = ? AND from_date = ?";

    let result = query(query_str)
        .bind(emp_no)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("Salary deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting salary: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}