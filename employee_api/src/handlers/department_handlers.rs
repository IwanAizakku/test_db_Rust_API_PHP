// src/handlers/department_handlers.rs
use axum::{extract::Path, response::IntoResponse, Json};
use sqlx::MySqlPool;
use std::sync::Arc;
use crate::models::department::Department;

pub async fn get_departments(db: Arc<MySqlPool>) -> impl IntoResponse {
    let departments = sqlx::query_as::<_, Department>("SELECT * FROM departments")
        .fetch_all(&*db)
        .await;
    
    match departments {
        Ok(dept_list) => Json(dept_list).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching departments").into_response(),
    }
}

pub async fn get_department_by_id(Path(dept_no): Path<String>, db: Arc<MySqlPool>) -> impl IntoResponse {
    let department = sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE dept_no = ?")
        .bind(dept_no)
        .fetch_optional(&*db)
        .await;
    
    match department {
        Ok(Some(dept)) => Json(dept).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "Department not found").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching department").into_response(),
    }
}
