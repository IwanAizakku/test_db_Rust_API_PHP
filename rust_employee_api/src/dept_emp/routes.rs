use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_emp::handlers::*;

pub fn create_dept_emp_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dept_emp_list_handler))
        .route("/{:id}", get(get_dept_emp_handler))
        .with_state(app_state)
}