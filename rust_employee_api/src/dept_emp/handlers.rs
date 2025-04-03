// dept_emp/handlers.rs
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use serde_json::{Value, from_value, to_value};

use crate::db::AppState;
use crate::dept_emp::models::DeptEmp;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use sqlx::{query, query_as};

// CRUD operations using POST
pub async fn dept_emp_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_dept_emp(&state, payload["dept_emp"].clone()).await,
        "read_all" => read_all_dept_emps(&state).await,
        "read" => read_dept_emp(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        "update" => update_dept_emp(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), payload["dept_emp"].clone()).await,
        "delete" => delete_dept_emp(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["dept_no"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_dept_emp(state: &Arc<AppState>, dept_emp_value: Value) -> Result<Json<String>, StatusCode> {
    let new_dept_emp: DeptEmp = from_value(dept_emp_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO dept_emp (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = query(query_str)
        .bind(new_dept_emp.emp_no)
        .bind(&new_dept_emp.dept_no)
        .bind(&new_dept_emp.from_date)
        .bind(&new_dept_emp.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_dept_emp).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating dept_emp: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_dept_emps(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM dept_emp LIMIT 10";

    let dept_emps = query_as::<_, DeptEmp>(query_str)
        .fetch_all(&state.db)
        .await;

    match dept_emps {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching dept_emps: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_dept_emp(state: &Arc<AppState>, emp_no: i32, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM dept_emp WHERE emp_no = ? AND dept_no = ?";
    let dept_emp = query_as::<_, DeptEmp>(query_str)
        .bind(emp_no)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match dept_emp {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_dept_emp(state: &Arc<AppState>, emp_no: i32, dept_no: String, dept_emp_value: Value) -> Result<Json<String>, StatusCode> {
    let mut updated_dept_emp: DeptEmp = from_value(dept_emp_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    //Use emp_no and dept_no from path parameter
    updated_dept_emp.emp_no = emp_no;
    updated_dept_emp.dept_no = dept_no.clone();

    let query_str = "UPDATE dept_emp SET from_date = ?, to_date = ? WHERE emp_no = ? AND dept_no = ?";

    let result = query(query_str)
        .bind(&updated_dept_emp.from_date)
        .bind(&updated_dept_emp.to_date)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(updated_dept_emp).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating dept_emp: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_dept_emp(state: &Arc<AppState>, emp_no: i32, dept_no: String) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM dept_emp WHERE emp_no = ? AND dept_no = ?";

    let result = query(query_str)
        .bind(emp_no)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("DeptEmp deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting dept_emp: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}