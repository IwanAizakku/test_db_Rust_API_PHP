use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDepartmentSchema {
    pub dept_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDepartmentSchema {
    pub dept_name: Option<String>,
}