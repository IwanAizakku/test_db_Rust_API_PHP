// src/main.rs
use std::sync::Arc;
use axum::{routing::get, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use sqlx::mysql::MySqlPool;
use std::net::SocketAddr;

mod db;
mod routes;
mod middleware;
mod models;

use db::connect_db;
use routes::employee_routes::employee_routes;
use routes::department_routes::department_routes;
use routes::salary_routes::salary_routes;
use middleware::auth::auth_layer;
use middleware::rate_limit::rate_limit_layer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting API Service...");

    let pool = connect_db().await.expect("Failed to connect to DB");
    let state = Arc::new(pool);

    let app = Router::new()
        .merge(employee_routes(state.clone()))
        .merge(department_routes(state.clone()))
        .merge(salary_routes(state.clone()))
        .layer(auth_layer())
        .layer(rate_limit_layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
