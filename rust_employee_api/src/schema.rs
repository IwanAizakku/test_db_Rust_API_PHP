use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEmployeeSchema {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub gender: String,  // Enum 'M' or 'F' for gender
    pub hire_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEmployeeSchema {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,  // Enum 'M' or 'F' for gender
    pub hire_date: Option<NaiveDate>,
}
