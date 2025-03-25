// titles/routes.rs
use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::db::AppState;
use crate::titles::handlers::*;
use crate::auth::Claims;
use crate::titles::models::*;
use chrono::NaiveDate;

pub fn create_title_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(move |claims: Claims, state: State<Arc<AppState>>, Json(new_title): Json<Title>| create_title_handler(claims, state, Json(new_title))))
        .route("/", get(move |claims: Claims, state: State<Arc<AppState>>| title_list_handler(claims, state)))
        .route("/{:emp_no}/{:title}/{:from_date}", get(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>| get_title_handler(claims, state, Path((emp_no, title, from_date))))
            .patch(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>, Json(updated_title): Json<Title>| edit_title_handler(claims, state, Path((emp_no, title, from_date)), Json(updated_title)))
            .delete(move |claims: Claims, state: State<Arc<AppState>>, Path((emp_no, title, from_date)): Path<(i32, String, NaiveDate)>| delete_title_handler(claims, state, Path((emp_no, title, from_date)))))
        .with_state(app_state)
}