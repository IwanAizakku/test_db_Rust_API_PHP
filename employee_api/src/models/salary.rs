// src/models/salary.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Salary {
    pub emp_no: i32,
    pub salary: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}