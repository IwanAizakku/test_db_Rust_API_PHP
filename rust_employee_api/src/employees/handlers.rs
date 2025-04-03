// Employees functions
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;

use crate::db::AppState;
use crate::employees::models::Employee;
use crate::auth::Claims;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};

use serde_json::{Value, from_value, to_value};
use sqlx::{query, query_as};

// CRUD operations using POST
pub async fn employee_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_employee(&state, payload["employee"].clone()).await,
        "read_all" => read_all_employees(&state).await,
        "read" => read_employee(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32).await,
        "update" => update_employee(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["employee"].clone()).await,
        "delete" => delete_employee(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_employee(state: &Arc<AppState>, employee_value: Value) -> Result<Json<String>, StatusCode> {
    let new_employee: Employee = from_value(employee_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO employees (emp_no, birth_date, first_name, last_name, gender, hire_date) VALUES (?, ?, ?, ?, ?, ?)";

    let result = query(query_str)
        .bind(new_employee.emp_no)
        .bind(&new_employee.birth_date)
        .bind(&new_employee.first_name)
        .bind(&new_employee.last_name)
        .bind(&new_employee.gender)
        .bind(&new_employee.hire_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_employee).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating employee: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_employees(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM employees LIMIT 10";

    let employees = query_as::<_, Employee>(query_str)
        .fetch_all(&state.db)
        .await;

    match employees {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching employees: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_employee(state: &Arc<AppState>, emp_no: i32) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM employees WHERE emp_no = ?";
    let employee = query_as::<_, Employee>(query_str)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match employee {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_employee(state: &Arc<AppState>, emp_no: i32, employee_value: Value) -> Result<Json<String>, StatusCode> {
    println!("Updating employee with emp_no: {}", emp_no);

    let mut updated_employee: Employee = match from_value(employee_value.clone()) {
        Ok(employee) => employee,
        Err(e) => {
            eprintln!("Error parsing employee_value: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    println!("Parsed updated_employee: {:?}", updated_employee);

    //Use emp_no from path parameter
    updated_employee.emp_no = emp_no;

    let query_str = "UPDATE employees SET birth_date = ?, first_name = ?, last_name = ?, gender = ?, hire_date = ? WHERE emp_no = ?";

    let result = query(query_str)
        .bind(&updated_employee.birth_date)
        .bind(&updated_employee.first_name)
        .bind(&updated_employee.last_name)
        .bind(&updated_employee.gender)
        .bind(&updated_employee.hire_date)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    println!("Database update result: {:?}", result);

    match result {
        Ok(_) => {
            let json_data = to_value(updated_employee).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating employee: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_employee(state: &Arc<AppState>, emp_no: i32) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM employees WHERE emp_no = ?";

    let result = query(query_str)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("Employee deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting employee: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}