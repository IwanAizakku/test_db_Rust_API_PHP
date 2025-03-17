use std::sync::Arc;
use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::net::SocketAddr;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!(" REST API Service test");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080)); // This binds to all network interfaces

    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("✅ Server started successfully at {}", addr);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

