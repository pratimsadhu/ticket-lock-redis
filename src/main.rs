use axum::{routing::get, Router};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener as TokioListener;

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let redis_client = Arc::new(config::redis_config::get_redis_client());

    let app = Router::new()
        .merge(routes::ticket::ticket_routes(redis_client.clone()))
        .route("/", get(|| async { "Ticket Lock Service Running" }));

    let listener = TokioListener::bind("0.0.0.0:3000").await.unwrap();
    let std_listener = listener.into_std().unwrap();
    println!("Server running on http://localhost:3000");

    axum::Server::from_tcp(std_listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
