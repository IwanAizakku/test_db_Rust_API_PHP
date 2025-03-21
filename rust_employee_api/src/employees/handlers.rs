// Employees functions
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;

use crate::db::AppState;
use crate::employees::models::Employee;

// Create Employee
pub async fn create_employee_handler(
    State(state): State<Arc<AppState>>,
    Json(new_employee): Json<Employee>,
) -> Result<Json<Employee>, StatusCode> {
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
        Ok(_) => Ok(Json(new_employee)),
        Err(e) => {
            eprintln!("Error creating employee: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get All Employees
pub async fn employee_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    let query = "SELECT * FROM employees";

    let employees = sqlx::query_as::<_, Employee>(query)
        .fetch_all(&state.db)
        .await;

    match employees {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("Error fetching employees: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get Employee by ID
pub async fn get_employee_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<Employee>, StatusCode> {
    let query = "SELECT * FROM employees WHERE emp_no = ?";
    let employee = sqlx::query_as::<_, Employee>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match employee {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Employee
pub async fn edit_employee_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
    Json(updated_employee): Json<Employee>,
) -> Result<Json<Employee>, StatusCode> {
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
        Ok(_) => Ok(Json(updated_employee)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete Employee
pub async fn delete_employee_handler(
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