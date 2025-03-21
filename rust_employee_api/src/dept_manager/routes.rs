use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_manager::handlers::*;

pub fn create_dept_manager_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dept_manager_list_handler))
        .route("/{:id}", get(get_dept_manager_handler))
        .with_state(app_state)
}