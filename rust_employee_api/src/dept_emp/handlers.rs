use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_emp::models::DeptEmp;

// List all Employees by department
pub async fn dept_emp_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeptEmp>>, StatusCode> {
    let query = "SELECT * FROM dept_emp";

    let dept_emps = sqlx::query_as::<_, DeptEmp>(query)
        .fetch_all(&state.db)
        .await;

    match dept_emps {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get an employee's department by ID
pub async fn get_dept_emp_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<DeptEmp>, StatusCode> {
    let query = "SELECT * FROM dept_emp WHERE emp_no = ?";
    let deptemp = sqlx::query_as::<_, DeptEmp>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match deptemp {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

