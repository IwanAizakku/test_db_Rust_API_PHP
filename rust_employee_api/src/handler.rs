use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{EmployeeModel, EmployeeModelResponse},
    schema::{CreateEmployeeSchema, FilterOptions, UpdateEmployeeSchema},
    AppState,
};

// Utility function to convert EmployeeModel to EmployeeModelResponse
fn filter_db_record(employee: &EmployeeModel) -> EmployeeModelResponse {
    EmployeeModelResponse {
        emp_no: employee.emp_no,
        first_name: employee.first_name.to_owned(),
        last_name: employee.last_name.to_owned(),
        birth_date: employee.birth_date,  // NaiveDate type
        gender: employee.gender.to_owned(),
        hire_date: employee.hire_date,  // NaiveDate type
    }
}

pub async fn employee_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Query to fetch employees, ordered by emp_no
    let employees = sqlx::query_as!(    
        EmployeeModel,
        r#"SELECT emp_no, first_name, last_name, birth_date, gender, hire_date FROM employees ORDER BY emp_no LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Convert the EmployeeModel to EmployeeModelResponse
    let employee_responses = employees
        .iter()
        .map(|employee| filter_db_record(&employee))
        .collect::<Vec<EmployeeModelResponse>>();

    // Create the JSON response with the list of employees
    let json_response = serde_json::json!({
        "status": "success",
        "results": employee_responses.len(),
        "employees": employee_responses
    });

    Ok(Json(json_response))
}

// Create function
pub async fn create_employee_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateEmployeeSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Generate a new emp_no for the employee (if auto-increment isn't used, you'd manually generate it)
    let emp_no = uuid::Uuid::new_v4().to_string(); // Or use auto-increment if emp_no is auto-generated in DB

    // Insert the new employee into the database
    let query_result = sqlx::query(r#"
        INSERT INTO employees (emp_no, first_name, last_name, birth_date, gender, hire_date) 
        VALUES (?, ?, ?, ?, ?, ?)
    "#)
    .bind(emp_no.clone())
    .bind(body.first_name.to_string())
    .bind(body.last_name.to_string())
    .bind(body.birth_date)
    .bind(body.gender.to_string())
    .bind(body.hire_date)
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    // Handle potential insertion errors
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Employee with that emp_no already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", err)})),
        ));
    }

    // Retrieve the newly inserted employee from the database
    let employee = sqlx::query_as!(EmployeeModel, r#"
        SELECT emp_no, first_name, last_name, birth_date, gender, hire_date 
        FROM employees 
        WHERE emp_no = ?
    "#, emp_no)
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )
    })?;

    // Convert the employee model to the response format
    let employee_response = serde_json::json!({
        "status": "success",
        "data": {
            "employee": filter_db_record(&employee)
        }
    });

    Ok(Json(employee_response))
}

// Get function
pub async fn get_employee_handler(
    Path(emp_no): Path<String>,  // emp_no is a string (could be UUID or integer depending on your design)
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        EmployeeModel,
        r#"SELECT emp_no, first_name, last_name, birth_date, gender, hire_date FROM employees WHERE emp_no = ?"#,
        emp_no
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(employee) => {
            // If the employee is found, return a successful response with employee details
            let employee_response = serde_json::json!({
                "status": "success",
                "data": {
                    "employee": filter_db_record(&employee)
                }
            });

            Ok(Json(employee_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            // If the employee is not found, return a 404 Not Found error
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Employee with emp_no: {} not found", emp_no)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => {
            // For any other errors, return an internal server error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ))
        }
    }
}

// Edit function
pub async fn edit_employee_handler(
    Path(emp_no): Path<String>,  // emp_no is a string (could be UUID or integer depending on your design)
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateEmployeeSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Fetch the employee data
    let query_result = sqlx::query_as!(
        EmployeeModel,
        r#"SELECT emp_no, first_name, last_name, birth_date, gender, hire_date FROM employees WHERE emp_no = ?"#,
        emp_no
    )
    .fetch_one(&data.db)
    .await;

    let employee = match query_result {
        Ok(employee) => employee,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Employee with emp_no: {} not found", emp_no)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    };

    // Perform the update with the provided fields
    let update_result = sqlx::query(
        r#"UPDATE employees SET first_name = ?, last_name = ?, birth_date = ?, gender = ?, hire_date = ? WHERE emp_no = ?"#,
    )
    .bind(body.first_name.clone().unwrap_or_else(|| employee.first_name.clone()))
    .bind(body.last_name.clone().unwrap_or_else(|| employee.last_name.clone()))
    .bind(body.birth_date.unwrap_or(employee.birth_date))  // Assuming birth_date can be updated
    .bind(body.gender.clone().unwrap_or_else(|| employee.gender.clone()))
    .bind(body.hire_date.unwrap_or(employee.hire_date))  // Assuming hire_date can be updated
    .bind(emp_no.clone())
    .execute(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )
    })?;

    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Employee with emp_no: {} not found", emp_no)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // Fetch the updated employee data
    let updated_employee = sqlx::query_as!(
        EmployeeModel,
        r#"SELECT emp_no, first_name, last_name, birth_date, gender, hire_date FROM employees WHERE emp_no = ?"#,
        emp_no
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )
    })?;

    let employee_response = serde_json::json!({
        "status": "success",
        "data": {
            "employee": filter_db_record(&updated_employee)
        }
    });

    Ok(Json(employee_response))
}

// Delete function
pub async fn delete_employee_handler(
    Path(emp_no): Path<String>,  // emp_no as a string (could be UUID or integer depending on your design)
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Perform the DELETE operation
    let query_result = sqlx::query!(
        r#"DELETE FROM employees WHERE emp_no = ?"#,
        emp_no
    )
    .execute(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )
    })?;

    // Check if any rows were affected
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Employee with emp_no: {} not found", emp_no)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // Return HTTP 204 No Content if the deletion is successful
    Ok(StatusCode::NO_CONTENT)
}

