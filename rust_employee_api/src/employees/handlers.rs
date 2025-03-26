// Employees functions
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;

use crate::db::AppState;
use crate::employees::models::Employee;
use crate::auth::Claims;

// Create Employee
pub async fn create_employee_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(new_employee): Json<Employee>,
) -> Result<Json<serde_json::Value>, StatusCode> { //Modified return type
    let query = "INSERT INTO employees (emp_no, birth_date, first_name, last_name, gender, hire_date) 
                                        VALUES (?, ?, ?, ?, ?, ?)";

    let result = sqlx::query(query)
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
            let json_data = serde_json::to_value(new_employee).unwrap();
            Ok(Json(json_data)) // Return raw JSON
        }
        Err(e) => {
            eprintln!("Error creating employee: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get All Employees
pub async fn employee_list_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> { //Modified return type
    let query = "SELECT * FROM employees";

    let employees = sqlx::query_as::<_, Employee>(query)
        .fetch_all(&state.db)
        .await;

    match employees {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            Ok(Json(json_data)) // Return raw JSON
        }
        Err(e) => {
            eprintln!("Error fetching employees: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get Employee by ID
pub async fn get_employee_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> { //Modified return type
    let query = "SELECT * FROM employees WHERE emp_no = ?";
    let employee = sqlx::query_as::<_, Employee>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match employee {
        Ok(data) => {
            let json_data = serde_json::to_value(data).unwrap();
            Ok(Json(json_data)) // Return raw JSON
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Employee
pub async fn edit_employee_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
    Json(updated_employee): Json<Employee>,
) -> Result<Json<serde_json::Value>, StatusCode> { //Modified return type
    let query = "UPDATE employees 
                                        SET birth_date = ?, first_name = ?, last_name = ?, gender = ?, hire_date = ? 
                                        WHERE emp_no = ?";

    let result = sqlx::query(query)
        .bind(&updated_employee.birth_date)
        .bind(&updated_employee.first_name)
        .bind(&updated_employee.last_name)
        .bind(&updated_employee.gender)
        .bind(&updated_employee.hire_date)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = serde_json::to_value(updated_employee).unwrap();
            Ok(Json(json_data)) // Return raw JSON
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete Employee
pub async fn delete_employee_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> StatusCode {
    let query = "DELETE FROM employees WHERE emp_no = ?";

    let result = sqlx::query(query)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}