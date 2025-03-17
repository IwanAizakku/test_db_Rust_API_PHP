// src/db.rs
use sqlx::{mysql::MySqlPool, Error};
use std::env;

pub async fn connect_db() -> Result<MySqlPool, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    println!("✅ Connected to the database");
    Ok(pool)
}
