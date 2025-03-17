// src/routes/employee_routes.rs
use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::handlers::employee_handlers::{get_employees, get_employee_by_id};

pub fn employee_routes(db: Arc<MySqlPool>) -> Router {
    Router::new()
        .route("/employees", get(get_employees))
        .route("/employees/:id", get(get_employee_by_id))
}
