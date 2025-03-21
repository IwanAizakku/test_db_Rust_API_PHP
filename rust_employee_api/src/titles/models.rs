use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Title {
    pub emp_no: i32,
    pub title: String,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
}