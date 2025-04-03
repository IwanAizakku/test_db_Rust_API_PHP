// titles/handlers.rs
use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use serde_json::{Value, from_value, to_value};

use crate::db::AppState;
use crate::titles::models::Title;
use crate::sodium::sodium_crypto::{encrypt_json, get_key};
use crate::auth::Claims;
use sqlx::{query, query_as};
use chrono::NaiveDate;

// CRUD operations using POST
pub async fn title_crud_handler(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<String>, StatusCode> {
    let action = payload["action"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    match action {
        "create" => create_title(&state, payload["title"].clone()).await,
        "read_all" => read_all_titles(&state).await,
        "read" => read_title(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["title_name"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?).await,
        "update" => update_title(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["title_name"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?, payload["title"].clone()).await,
        "delete" => delete_title(&state, payload["emp_no"].as_i64().ok_or(StatusCode::BAD_REQUEST)? as i32, payload["title_name"].as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string(), NaiveDate::parse_from_str(payload["from_date"].as_str().ok_or(StatusCode::BAD_REQUEST)?, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_title(state: &Arc<AppState>, title_value: Value) -> Result<Json<String>, StatusCode> {
    let new_title: Title = from_value(title_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query_str = "INSERT INTO titles (emp_no, title, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = query(query_str)
        .bind(new_title.emp_no)
        .bind(&new_title.title)
        .bind(&new_title.from_date)
        .bind(new_title.to_date.as_ref())
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(new_title).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error creating title: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_all_titles(state: &Arc<AppState>) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM titles LIMIT 10";

    let titles = query_as::<_, Title>(query_str)
        .fetch_all(&state.db)
        .await;

    match titles {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error fetching titles: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn read_title(state: &Arc<AppState>, emp_no: i32, title_name: String, from_date: NaiveDate) -> Result<Json<String>, StatusCode> {
    let query_str = "SELECT * FROM titles WHERE emp_no = ? AND title = ? AND from_date = ?";
    let title = query_as::<_, Title>(query_str)
        .bind(emp_no)
        .bind(title_name)
        .bind(from_date)
        .fetch_one(&state.db)
        .await;

    match title {
        Ok(data) => {
            let json_data = to_value(data).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_title(state: &Arc<AppState>, emp_no: i32, title_name: String, from_date: NaiveDate, title_value: Value) -> Result<Json<String>, StatusCode> {
    let mut updated_title: Title = from_value(title_value).map_err(|_| StatusCode::BAD_REQUEST)?;

    //Use emp_no, title, and from_date from path parameter
    updated_title.emp_no = emp_no;
    updated_title.title = title_name;
    updated_title.from_date = from_date;

    let query_str = "UPDATE titles SET to_date = ? WHERE emp_no = ? AND title = ? AND from_date = ?";

    let result = query(query_str)
        .bind(updated_title.to_date.as_ref())
        .bind(emp_no)
        .bind(updated_title.title.clone())
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            let json_data = to_value(updated_title).unwrap();
            let key = get_key();
            let encrypted_data = encrypt_json(&json_data, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(encrypted_data))
        }
        Err(e) => {
            eprintln!("Error updating title: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_title(state: &Arc<AppState>, emp_no: i32, title_name: String, from_date: NaiveDate) -> Result<Json<String>, StatusCode> {
    let query_str = "DELETE FROM titles WHERE emp_no = ? AND title = ? AND from_date = ?";

    let result = query(query_str)
        .bind(emp_no)
        .bind(title_name)
        .bind(from_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(String::from("Title deleted successfully"))),
        Err(e) => {
            eprintln!("Error deleting title: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}