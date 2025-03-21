use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::titles::models::Title;

pub async fn titles_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Title>>, StatusCode> {
    let query = "SELECT * FROM titles";

    let titles = sqlx::query_as::<_, Title>(query)
        .fetch_all(&state.db)
        .await;

    match titles {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Employee's title by ID
pub async fn get_titles_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<Title>, StatusCode> {
    let query = "SELECT * FROM titles WHERE emp_no = ?";
    let title = sqlx::query_as::<_, Title>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match title {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}