// src/main.rs
mod db;
mod employees;
mod departments;
mod dept_manager;
mod dept_emp;
mod titles;
mod salaries;

use std::sync::Arc;
use axum::{
    Router,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

use crate::db::AppState;
use crate::employees::routes as employee_routes;
use crate::departments::routes as department_routes;
use crate::dept_manager::routes as dept_manager_routes;
use crate::dept_emp::routes as dept_emp_routes;
use crate::titles::routes as titles_routes;
use crate::salaries::routes as salaries_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::create_pool().await;
    let app_state = Arc::new(AppState { db: pool.clone() });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let employee_routes = employee_routes::create_employee_routes(app_state.clone());
    let department_routes = department_routes::create_department_routes(app_state.clone());
    let dept_manager_routes = dept_manager_routes::create_dept_manager_routes(app_state.clone());
    let dept_emp_routes = dept_emp_routes::create_dept_emp_routes(app_state.clone());
    let titles_routes = titles_routes::create_titles_routes(app_state.clone());
    let salaries_routes = salaries_routes::create_salaries_routes(app_state.clone());

    let app = Router::new()
        .nest_service("/employees", employee_routes)
        .nest_service("/departments", department_routes)
        .nest_service("/dept_manager", dept_manager_routes)
        .nest_service("/dept_emp", dept_emp_routes)
        .nest_service("/titles", titles_routes)
        .nest_service("/salaries", salaries_routes)
        .layer(cors)
        .with_state(app_state);

    println!("🚀 Server started successfully!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}