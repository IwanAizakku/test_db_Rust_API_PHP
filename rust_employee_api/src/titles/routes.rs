use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::titles::handlers::*;

pub fn create_titles_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(titles_list_handler))
        .route("/{:id}", get(get_titles_handler))
        .with_state(app_state)
}