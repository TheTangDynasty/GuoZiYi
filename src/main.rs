pub mod entity;
pub mod routes;
pub mod service;

use std::env;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use sea_orm::Database;

use crate::routes::process::add_process;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file is not found");
    let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
    let db_connect = Database::connect(db_url).await.expect("connect db failed");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/eventTrack/online", post(handler))
        .route("/eventTrack/offline", post(handler))
        .route("/eventTrack/token", post(handler))
        .route("/api/process/create", get(add_process));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
