pub mod routes;
pub mod schedule;
pub mod service;
pub mod utils;

use std::env;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use redis::Commands;
use sea_orm::Database;
use tera::Tera;

use crate::{
    routes::{
        heartbeat::attach_heart_beat,
        render::{main_page, route_page},
        token::{alive_token, create_token, record},
    },
    schedule::clear_token::clear_token_timer,
    utils::app_state::AppState,
};

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file is not found");
    let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
    let env = env::var("RUST_ENV").expect("RUST_ENV is not set in .env file");

    let db_connect = Database::connect(db_url).await.expect("connect db failed");

    db_connect.ping().await.expect("ping db failed");

    let redis = redis::Client::open("redis://127.0.0.1:6379/").unwrap();

    let tera = Tera::new("templates/**/*").unwrap();

    clear_token_timer(redis.clone(), db_connect.clone());

    let state = AppState {
        db: db_connect,
        redis: redis,
        tera: tera,
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler222))
        .route("/eventTrack/online", post(handler222))
        .route("/eventTrack/offline", post(record))
        .route("/eventTrack/heartbeat", get(attach_heart_beat))
        .route("/eventTrack/token", post(create_token))
        .route("/token/list", get(alive_token))
        .route("/main", get(main_page))
        .route("/route/:route_name", get(route_page))
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler222() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
