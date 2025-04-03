// dept_manager/routes.rs
use axum::{
    routing::post,
    Router,
    extract::State,
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_manager::handlers::dept_manager_crud_handler;
use crate::auth::Claims;
use serde_json::Value;

pub fn create_dept_manager_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(payload): Json<Value>| dept_manager_crud_handler(claims, state, Json(payload))))
        .with_state(app_state)
}