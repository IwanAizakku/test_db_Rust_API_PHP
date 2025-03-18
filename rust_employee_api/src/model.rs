use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct EmployeeModel {
    pub emp_no: i32,                      // Corresponding to emp_no (INT)
    pub birth_date: NaiveDate,             // Corresponding to birth_date (DATE)
    pub first_name: String,                // Corresponding to first_name (VARCHAR(14))
    pub last_name: String,                 // Corresponding to last_name (VARCHAR(16))
    pub gender: String,                    // Corresponding to gender (ENUM('M', 'F'))
    pub hire_date: NaiveDate,              // Corresponding to hire_date (DATE)
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EmployeeModelResponse {
    pub emp_no: i32,                      // Corresponding to emp_no (INT)
    pub birth_date: NaiveDate,             // Corresponding to birth_date (DATE)
    pub first_name: String,                // Corresponding to first_name (VARCHAR(14))
    pub last_name: String,                 // Corresponding to last_name (VARCHAR(16))
    pub gender: String,                    // Corresponding to gender (ENUM('M', 'F'))
    pub hire_date: NaiveDate,              // Corresponding to hire_date (DATE)
}

