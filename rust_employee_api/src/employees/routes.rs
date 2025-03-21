use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::employees::handlers::*;

pub fn create_employee_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_employee_handler))
        .route("/", get(employee_list_handler))
        .route("/{:id}", get(get_employee_handler).patch(edit_employee_handler).delete(delete_employee_handler))
        .with_state(app_state)
}