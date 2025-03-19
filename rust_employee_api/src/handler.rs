use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;

use crate::model::{Employee, Department, DeptManager, DeptEmp, Title, Salary};
use crate::AppState;

// =========================
// EMPLOYEES HANDLERS
// =========================

// Create Employee
pub async fn create_employee_handler(
    State(state): State<Arc<AppState>>,
    Json(new_employee): Json<Employee>,
) -> Result<Json<Employee>, StatusCode> {
    let query = "INSERT INTO employees (emp_no, birth_date, first_name, last_name, gender, hire_date) 
                 VALUES (?, ?, ?, ?, ?, ?)";
    
    let result = sqlx::query(query)
        .bind(new_employee.emp_no)
        .bind(&new_employee.birth_date)
        .bind(&new_employee.first_name)
        .bind(&new_employee.last_name)
        .bind(&new_employee.gender)
        .bind(&new_employee.hire_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(new_employee)),
        Err(e) => {
            eprintln!("Error fetching employees: {:?}", e);  // This will print the error to your console
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        
    }
}

// Get All Employees
pub async fn employee_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    let query = "SELECT * FROM employees";

    // Execute the query and handle possible errors
    let employees = sqlx::query_as::<_, Employee>(query)
        .fetch_all(&state.db)
        .await;

    match employees {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            // Log the specific error for debugging purposes
            eprintln!("Error fetching employees: {:?}", e);  // This will print the error to your console
            
            // Respond with an internal server error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


// Get Employee by ID
pub async fn get_employee_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> Result<Json<Employee>, StatusCode> {
    let query = "SELECT * FROM employees WHERE emp_no = ?";
    let employee = sqlx::query_as::<_, Employee>(query)
        .bind(emp_no)
        .fetch_one(&state.db)
        .await;

    match employee {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Employee
pub async fn edit_employee_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
    Json(updated_employee): Json<Employee>,
) -> Result<Json<Employee>, StatusCode> {
    let query = "UPDATE employees 
                 SET birth_date = ?, first_name = ?, last_name = ?, gender = ?, hire_date = ? 
                 WHERE emp_no = ?";

    let result = sqlx::query(query)
        .bind(&updated_employee.birth_date)
        .bind(&updated_employee.first_name)
        .bind(&updated_employee.last_name)
        .bind(&updated_employee.gender)
        .bind(&updated_employee.hire_date)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(updated_employee)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete Employee
pub async fn delete_employee_handler(
    State(state): State<Arc<AppState>>,
    Path(emp_no): Path<i32>,
) -> StatusCode {
    let query = "DELETE FROM employees WHERE emp_no = ?";

    let result = sqlx::query(query)
        .bind(emp_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// =========================
// DEPARTMENTS HANDLERS
// =========================

// Create Department
pub async fn create_department_handler(
    State(state): State<Arc<AppState>>,
    Json(new_department): Json<Department>,
) -> Result<Json<Department>, StatusCode> {
    let query = "INSERT INTO departments (dept_no, dept_name) VALUES (?, ?)";
    
    let result = sqlx::query(query)
        .bind(&new_department.dept_no)
        .bind(&new_department.dept_name)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(Json(new_department)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get All Departments
pub async fn department_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Department>>, StatusCode> {
    let query = "SELECT * FROM departments";
    let departments = sqlx::query_as::<_, Department>(query)
        .fetch_all(&state.db)
        .await;

    match departments {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get Department by ID
pub async fn get_department_handler(
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
) -> Result<Json<Department>, StatusCode> {
    let query = "SELECT * FROM departments WHERE dept_no = ?";
    let department = sqlx::query_as::<_, Department>(query)
        .bind(dept_no)
        .fetch_one(&state.db)
        .await;

    match department {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Update Department
pub async fn edit_department_handler(
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
    Json(updated_department): Json<Department>,
) -> Result<Json<Department>, StatusCode> {
    let query = "UPDATE departments 
                 SET dept_name = ? WHERE dept_no = ?";
    let result = sqlx::query(query)
        .bind(&updated_department.dept_name)
        .bind(dept_no)
        .execute(&state.db)
        .await;
    match result {
        Ok(_) => Ok(Json(updated_department)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_department_handler(
    State(state): State<Arc<AppState>>,
    Path(dept_no): Path<String>,
) -> StatusCode {
    let query = "DELETE FROM departments WHERE dept_no = ?";
    let result = sqlx::query(query)
        .bind(dept_no)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// =========================
// DEPARTMENT MANAGER HANDLERS
// =========================

// Assign Department Manager
pub async fn assign_dept_manager_handler(
    State(state): State<Arc<AppState>>,
    Json(dept_manager): Json<DeptManager>,
) -> StatusCode {
    let query = "INSERT INTO dept_manager (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(dept_manager.emp_no)
        .bind(&dept_manager.dept_no)
        .bind(&dept_manager.from_date)
        .bind(&dept_manager.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// =========================
// DEPARTMENT EMPLOYEE HANDLERS
// =========================

// Assign Employee to Department
pub async fn assign_dept_emp_handler(
    State(state): State<Arc<AppState>>,
    Json(dept_emp): Json<DeptEmp>,
) -> StatusCode {
    let query = "INSERT INTO dept_emp (emp_no, dept_no, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(dept_emp.emp_no)
        .bind(&dept_emp.dept_no)
        .bind(&dept_emp.from_date)
        .bind(&dept_emp.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// =========================
// TITLES HANDLERS
// =========================

// Assign Title to Employee
pub async fn assign_title_handler(
    State(state): State<Arc<AppState>>,
    Json(title): Json<Title>,
) -> StatusCode {
    let query = "INSERT INTO titles (emp_no, title, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(title.emp_no)
        .bind(&title.title)
        .bind(&title.from_date)
        .bind(&title.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// =========================
// SALARY HANDLERS
// =========================

// Assign Salary to Employee
pub async fn assign_salary_handler(
    State(state): State<Arc<AppState>>,
    Json(salary): Json<Salary>,
) -> StatusCode {
    let query = "INSERT INTO salaries (emp_no, salary, from_date, to_date) VALUES (?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(salary.emp_no)
        .bind(salary.salary)
        .bind(&salary.from_date)
        .bind(&salary.to_date)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

