use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::departments::handlers::*;

pub fn create_department_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_department_handler))
        .route("/", get(department_list_handler))
        .route("/{:id}", get(get_department_handler).patch(edit_department_handler).delete(delete_department_handler))
        .with_state(app_state)
}