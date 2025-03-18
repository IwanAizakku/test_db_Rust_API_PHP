use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_employee_handler, delete_employee_handler, edit_employee_handler, get_employee_handler,
        employee_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/employees", post(create_employee_handler))
        .route("/employees", get(employee_list_handler))
        .route(
            "/employees/:id",
            get(get_employee_handler)
                .patch(edit_employee_handler)
                .delete(delete_employee_handler),
        )
        .with_state(app_state)
}
