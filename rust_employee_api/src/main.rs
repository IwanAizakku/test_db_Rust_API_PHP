mod db;
mod employees;
mod departments;
mod dept_manager;
mod dept_emp;
mod titles;
mod salaries;
mod auth;
mod rate_limit;
mod sodium;

use std::sync::Arc;
use axum::{
    extract::{FromRequestParts, Query},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    http::request::Parts,
    Router, Json,
};
use futures::future::ready;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use crate::auth::{Claims, validate_jwt};
use crate::db::AppState;
use serde_json::{json, Value};
use std::future::Future;

use crate::employees::routes as employee_routes;
use crate::departments::routes as department_routes;
use crate::dept_manager::routes as dept_manager_routes;
use crate::dept_emp::routes as dept_emp_routes;
use crate::titles::routes as titles_routes;
use crate::salaries::routes as salaries_routes;

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        ready(
            parts.headers.get(axum::http::header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
                .filter(|header_str| header_str.starts_with("Bearer "))
                .map(|header_str| header_str.trim_start_matches("Bearer "))
                .map_or(Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"})))), |token| {
                    match validate_jwt(token) {
                        Ok(claims) => Ok(claims),
                        Err(e) => {
                            match e.kind() {
                                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Token expired"})))),
                                _ => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"})))),
                            }
                        }
                    }
                }),
        )
    }
}

async fn decrypt_handler(Query(params): Query<std::collections::HashMap<String, String>>) -> Result<Json<Value>, StatusCode> {
    let mut encrypted_data = params.get("data").ok_or(StatusCode::BAD_REQUEST)?.clone(); // Clone the string so we can modify it.
    encrypted_data = encrypted_data.replace(" ", "+"); // Replace spaces with '+'
    println!("Modified Encrypted Data: {}", encrypted_data); // print the modified string.
    match crate::sodium::sodium_crypto::decrypt_string(&encrypted_data) {
        Ok(value) => Ok(Json(value)),
        Err(e) => {
            eprintln!("Decryption error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    sodiumoxide::init().expect("sodiumoxide::init failed"); // Initialize Sodium

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
    let titles_routes = titles_routes::create_title_routes(app_state.clone());
    let salaries_routes = salaries_routes::create_salary_routes(app_state.clone());

    let app = Router::new()
        .nest_service("/employees", employee_routes)
        .nest_service("/departments", department_routes)
        .nest_service("/dept_manager", dept_manager_routes)
        .nest_service("/dept_emp", dept_emp_routes)
        .nest_service("/titles", titles_routes)
        .nest_service("/salaries", salaries_routes)
        .route("/decrypt", axum::routing::get(decrypt_handler)) // Add the new route
        .layer(cors)
        .with_state(app_state.clone())
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), crate::rate_limit::rate_limit::rate_limit_middleware));

    println!("🚀 Server started successfully!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}