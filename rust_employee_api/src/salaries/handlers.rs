use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::salaries::models::Salary;

pub async fn salaries_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Salary>>, StatusCode> {
    let query = "SELECT * FROM salaries";

    let salaries = sqlx::query_as::<_, Salary>(query)
        .fetch_all(&state.db)
        .await;

    match salaries {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Employee's salary by ID
pub async fn get_salaries_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<Salary>, StatusCode> {
    let query = "SELECT * FROM salaries WHERE emp_no = ?";
    let salary = sqlx::query_as::<_, Salary>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match salary {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}