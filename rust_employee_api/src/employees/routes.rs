use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::employees::handlers::*;
use crate::auth::Claims;
use crate::employees::models::*;

pub fn create_employee_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(new_employee): Json<Employee>| create_employee_handler(claims, state, Json(new_employee))))
        .route("/", get(move |claims: Claims, state: State<Arc<AppState>>| employee_list_handler(claims, state)))
        .route("/{:id}", get(move |claims: Claims, state: State<Arc<AppState>>, Path(emp_no): Path<i32>| get_employee_handler(claims, state, Path(emp_no)))
            .patch(move |claims: Claims, state: State<Arc<AppState>>, Path(emp_no): Path<i32>, Json(updated_employee): Json<Employee>| edit_employee_handler(claims, state, Path(emp_no), Json(updated_employee)))
            .delete(move |claims: Claims, state: State<Arc<AppState>>, Path(emp_no): Path<i32>| delete_employee_handler(claims, state, Path(emp_no))))
        .with_state(app_state)
}