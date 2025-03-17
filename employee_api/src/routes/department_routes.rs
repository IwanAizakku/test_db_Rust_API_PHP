// src/routes/department_routes.rs
use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::handlers::department_handlers::{get_departments, get_department_by_id};

pub fn department_routes(db: Arc<MySqlPool>) -> Router {
    Router::new()
        .route("/departments", get(get_departments))
        .route("/departments/:id", get(get_department_by_id))
}
