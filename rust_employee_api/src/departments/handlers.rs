// Departments functions
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use serde_json::{Value, from_value, to_value};

use crate::db::AppState;
use crate::departments::models::Department;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use sqlx::{query, query_as};

// CRUD operations using POST
pub async fn department_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_department(&state, payload["department"].clone()).await,
        "read_all" => read_all_departments(&state).await,
        "read" => read_department(&state, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        "update" => update_department(&state, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), payload["department"].clone()).await,
        "delete" => delete_department(&state, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_department(state: &Arc<AppState>, department_value: Value) -> Result<Json<String>, StatusCode> {
    let new_department: Department = from_value(department_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO departments (dept_no, dept_name) VALUES (?, ?)";

    let result = query(query_str)
        .bind(&new_department.dept_no)
        .bind(&new_department.dept_name)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_department).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating department: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_departments(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM departments LIMIT 10";

    let departments = query_as::<_, Department>(query_str)
        .fetch_all(&state.db)
        .await;

    match departments {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching departments: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_department(state: &Arc<AppState>, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM departments WHERE dept_no = ?";
    let department = query_as::<_, Department>(query_str)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match department {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_department(state: &Arc<AppState>, dept_no: String, department_value: Value) -> Result<Json<String>, StatusCode> {
    let mut updated_department: Department = from_value(department_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    updated_department.dept_no = dept_no.clone(); // Use dept_no from path parameter

    let query_str = "UPDATE departments SET dept_name = ? WHERE dept_no = ?";

    let result = query(query_str)
        .bind(&updated_department.dept_name)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(updated_department).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating department: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_department(state: &Arc<AppState>, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM departments WHERE dept_no = ?";

    let result = query(query_str)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("Department deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting department: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}