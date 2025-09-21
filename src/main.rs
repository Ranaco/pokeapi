mod db;
mod models;
mod handlers;
mod errors;
mod routes;
mod utils;
mod client;

use axum::serve;
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    utils::init_tracing();

    let app = routes::create_router().await;

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    println!("Starting server at {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap()
}

