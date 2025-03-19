use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::handler::*;
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // Employee Routes
        .route("/employees", post(create_employee_handler))
        .route("/employees", get(employee_list_handler))
        .route(
            "/employees/:id",
            get(get_employee_handler)
                .patch(edit_employee_handler)
                .delete(delete_employee_handler),
        )
        
        // Department Routes
        .route("/departments", post(create_department_handler))
        .route("/departments", get(department_list_handler))
        .route(
            "/departments/:id",
            get(get_department_handler)
                .patch(edit_department_handler)
                .delete(delete_department_handler),
        )

        // Department Manager Routes
        .route("/dept_manager", post(assign_dept_manager_handler))

        // Department Employee Routes
        .route("/dept_emp", post(assign_dept_emp_handler))

        // Title Routes
        .route("/titles", post(assign_title_handler))

        // Salary Routes
        .route("/salaries", post(assign_salary_handler))

        .with_state(app_state)
}
