use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeptManager {
    pub emp_no: i32,
    pub dept_no: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}