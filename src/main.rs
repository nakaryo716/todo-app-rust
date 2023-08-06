use anyhow::Context;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},    
};
use thiserror::Error;


#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn creat(&self, payload: CreatTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>; 
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreatTodo {
    text: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: String,
    completed: bool,    
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = creat_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn creat_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(creat_user))
    
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn creat_user(Json(payload): Json<CreatUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct CreatUser {
    username: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct User {
    id: i64,
    username: String,
}


#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use mime::APPLICATION_JSON;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = creat_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello World!");

    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"username": "田中　太郎"}"#))
            .unwrap();

        let res = creat_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body = String::from_utf8(bytes.to_vec()).unwrap();

        let user: User = serde_json::from_str(&body).expect("cannot cover User instans");

        assert_eq!(
            user,
            User {
                id: 1337,
                username: "田中　太郎".to_string(),
            }
        );
    }
}