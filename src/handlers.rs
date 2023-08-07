use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::repositories::{CreatTodo, TodoRepository};

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreatTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.creat(payload);

    (StatusCode::CREATED, Json(todo))
}