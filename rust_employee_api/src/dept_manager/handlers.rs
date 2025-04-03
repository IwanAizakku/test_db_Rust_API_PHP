// dept_manager/handlers.rs
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use serde_json::{Value, from_value, to_value};

use crate::db::AppState;
use crate::dept_manager::models::DeptManager;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use sqlx::{query, query_as};

// CRUD operations using POST
pub async fn dept_manager_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_dept_manager(&state, payload["dept_manager"].clone()).await,
        "read_all" => read_all_dept_managers(&state).await,
        "read" => read_dept_manager(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        "update" => update_dept_manager(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), payload["dept_manager"].clone()).await,
        "delete" => delete_dept_manager(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_dept_manager(state: &Arc<AppState>, dept_manager_value: Value) -> Result<Json<String>, StatusCode> {
    let new_dept_manager: DeptManager = from_value(dept_manager_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO dept_manager (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = query(query_str)
        .bind(new_dept_manager.emp_no)
        .bind(&new_dept_manager.dept_no)
        .bind(&new_dept_manager.from_date)
        .bind(&new_dept_manager.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_dept_manager).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating dept_manager: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_dept_managers(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM dept_manager LIMIT 10";

    let dept_managers = query_as::<_, DeptManager>(query_str)
        .fetch_all(&state.db)
        .await;

    match dept_managers {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching dept_managers: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_dept_manager(state: &Arc<AppState>, emp_no: i32, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM dept_manager WHERE emp_no = ? AND dept_no = ?";
    let dept_manager = query_as::<_, DeptManager>(query_str)
        .bind(emp_no)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match dept_manager {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_dept_manager(state: &Arc<AppState>, emp_no: i32, dept_no: String, dept_manager_value: Value) -> Result<Json<String>, StatusCode> {
    let mut updated_dept_manager: DeptManager = from_value(dept_manager_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    //Use emp_no and dept_no from path parameter
    updated_dept_manager.emp_no = emp_no;
    updated_dept_manager.dept_no = dept_no.clone();

    let query_str = "UPDATE dept_manager SET from_date = ?, to_date = ? WHERE emp_no = ? AND dept_no = ?";

    let result = query(query_str)
        .bind(&updated_dept_manager.from_date)
        .bind(&updated_dept_manager.to_date)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(updated_dept_manager).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating dept_manager: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_dept_manager(state: &Arc<AppState>, emp_no: i32, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM dept_manager WHERE emp_no = ? AND dept_no = ?";

    let result = query(query_str)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("DeptManager deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting dept_manager: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}