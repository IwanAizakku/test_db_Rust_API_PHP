// salaries/routes.rs
use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::salaries::handlers::*;
use crate::auth::Claims;
use crate::salaries::models::*;
use chrono::NaiveDate;

pub fn create_salary_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(new_salary): Json<Salary>| create_salary_handler(claims, state, Json(new_salary))))
        .route("/", get(move |claims: Claims, state: State<Arc<AppState>>| salary_list_handler(claims, state)))
        .route("/{:emp_no}/{:from_date}", get(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, from_date)): Path<(i32, NaiveDate)>| get_salary_handler(claims, state, Path((emp_no, from_date))))
            .patch(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, from_date)): Path<(i32, NaiveDate)>, Json(updated_salary): Json<Salary>| edit_salary_handler(claims, state, Path((emp_no, from_date)), Json(updated_salary)))
            .delete(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, from_date)): Path<(i32, NaiveDate)>| delete_salary_handler(claims, state, Path((emp_no, from_date)))))
        .with_state(app_state)
}