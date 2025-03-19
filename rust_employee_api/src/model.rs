use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

// Employees Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub emp_no: i32,
    pub birth_date: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub hire_date: NaiveDate,
}

// Departments Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub dept_no: String,
    pub dept_name: String,
}

// Department Manager Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeptManager {
    pub emp_no: i32,
    pub dept_no: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

// Department Employee Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeptEmp {
    pub emp_no: i32,
    pub dept_no: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

// Titles Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Title {
    pub emp_no: i32,
    pub title: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

// Salaries Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Salary {
    pub emp_no: i32,
    pub salary: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}
