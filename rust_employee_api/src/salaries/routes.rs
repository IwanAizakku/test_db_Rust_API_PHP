use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::salaries::handlers::*;

pub fn create_salaries_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(salaries_list_handler))
        .route("/{:id}", get(get_salaries_handler))
        .with_state(app_state)
}