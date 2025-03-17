// src/handlers/employee_handlers.rs
use axum::{extract::Path, response::IntoResponse, Json};
use sqlx::MySqlPool;
use std::sync::Arc;
use crate::models::employee::Employee;

pub async fn get_employees(db: Arc<MySqlPool>) -> impl IntoResponse {
    let employees = sqlx::query_as::<_, Employee>("SELECT * FROM employees")
        .fetch_all(&*db)
        .await;
    
    match employees {
        Ok(emp_list) => Json(emp_list).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching employees").into_response(),
    }
}

pub async fn get_employee_by_id(Path(emp_no): Path<i32>, db: Arc<MySqlPool>) -> impl IntoResponse {
    let employee = sqlx::query_as::<_, Employee>("SELECT * FROM employees WHERE emp_no = ?")
        .bind(emp_no)
        .fetch_optional(&*db)
        .await;
    
    match employee {
        Ok(Some(emp)) => Json(emp).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "Employee not found").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching employee").into_response(),
    }
}
