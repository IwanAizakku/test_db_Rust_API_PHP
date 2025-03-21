use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_manager::models::DeptManager;

// List all Managers by department
pub async fn dept_manager_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeptManager>>, StatusCode> {
    let query = "SELECT * FROM dept_manager";

    let dept_managers = sqlx::query_as::<_, DeptManager>(query)
        .fetch_all(&state.db)
        .await;

    match dept_managers {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get a manager's department by ID
pub async fn get_dept_manager_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<DeptManager>, StatusCode> {
    let query = "SELECT * FROM dept_manager WHERE emp_no = ?";
    let deptmanager = sqlx::query_as::<_, DeptManager>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match deptmanager {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}