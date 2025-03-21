use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    pub db: MySqlPool,
}

pub async fn create_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connected to database!");
            pool
        }
        Err(err) => {
            println!("💀 connection failed: {:?}", err);
            std::process::exit(1);
        }
    }
}