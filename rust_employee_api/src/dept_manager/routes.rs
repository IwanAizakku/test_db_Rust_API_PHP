// dept_manager/routes.rs
use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::dept_manager::handlers::*;
use crate::auth::Claims;
use crate::dept_manager::models::*;

pub fn create_dept_manager_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(new_dept_manager): Json<DeptManager>| create_dept_manager_handler(claims, state, Json(new_dept_manager))))
        .route("/", get(move |claims: Claims, state: State<Arc<AppState>>| dept_manager_list_handler(claims, state)))
        .route("/{:emp_no}/{:dept_no}", get(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, dept_no)): Path<(i32, String)>| get_dept_manager_handler(claims, state, Path((emp_no, dept_no))))
            .patch(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, dept_no)): Path<(i32, String)>, Json(updated_dept_manager): Json<DeptManager>| edit_dept_manager_handler(claims, state, Path((emp_no, dept_no)), Json(updated_dept_manager)))
            .delete(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, dept_no)): Path<(i32, String)>| delete_dept_manager_handler(claims, state, Path((emp_no, dept_no)))))
        .with_state(app_state)
}