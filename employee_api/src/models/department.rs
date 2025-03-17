// src/models/department.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub dept_no: String,
    pub dept_name: String,
}