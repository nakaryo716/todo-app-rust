mod handlers;
mod repositories;


use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::{env,sync::Arc};

use repositories::{TodoRepository, TodoRepositoryForDb};
use sqlx::PgPool;
use dotenv::dotenv;
use handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo };



#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL")
        .expect("undifind [DATABASE_URL]");
    tracing::debug!("start connect database...");
    
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is {}", database_url));
    let repository = TodoRepositoryForDb::new(pool.clone());

    let app = creat_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn creat_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route("/todos/:id", get(find_todo::<T>).delete(delete_todo::<T>).patch(update_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}



async fn root() -> &'static str {
    "Hello World!"
}



