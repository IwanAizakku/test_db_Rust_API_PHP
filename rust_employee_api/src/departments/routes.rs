use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::departments::handlers::*;
use crate::auth::Claims;
use crate::departments::models::*;

pub fn create_department_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(new_department): Json<Department>| create_department_handler(claims, state, Json(new_department))))
        .route("/", get(move |claims: Claims, state: State<Arc<AppState>>| department_list_handler(claims, state)))
        .route("/{:dept_no}", get(move |claims: Claims, state: State<Arc<AppState>>, Path(dept_no): Path<String>| get_department_handler(claims, state, Path(dept_no)))
            .patch(move |claims: Claims, state: State<Arc<AppState>>, Path(dept_no): Path<String>, Json(updated_department): Json<Department>| edit_department_handler(claims, state, Path(dept_no), Json(updated_department)))
            .delete(move |claims: Claims, state: State<Arc<AppState>>, Path(dept_no): Path<String>| delete_department_handler(claims, state, Path(dept_no))))
        .with_state(app_state)
}