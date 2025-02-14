use axum::{routing::get, Router};
use colored::*;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener as TokioListener;

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize Redis client
    let redis_client = match config::redis_config::get_redis_client() {
        Ok(client) => Arc::new(client),
        Err(err) => {
            eprintln!(
                "{} Failed to initialize Redis: {}",
                "FATAL:".red().bold(),
                err.to_string().red()
            );
            return; // Exit the program if Redis is not available
        }
    };

    // Set up router with routes
    let app = Router::new()
        .merge(routes::ticket::ticket_routes(redis_client.clone()))
        .route("/", get(|| async { "Ticket Lock Service Running" }));

    // Set up server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = match TokioListener::bind(&addr).await {
        Ok(l) => l,
        Err(err) => {
            eprintln!(
                "{} Failed to bind to address {}: {}",
                "FATAL:".red().bold(),
                addr.yellow(),
                err.to_string().red()
            );
            return;
        }
    };

    let std_listener = match listener.into_std() {
        Ok(l) => l,
        Err(err) => {
            eprintln!(
                "{} Failed to convert listener: {}",
                "FATAL:".red().bold(),
                err.to_string().red()
            );
            return;
        }
    };

    println!(
        "{} Server running on {}",
        "INFO:".green().bold(),
        format!("http://localhost:{}", port).cyan()
    );

    // Start the server
    if let Err(err) = axum::Server::from_tcp(std_listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
    {
        eprintln!(
            "{} Server error: {}",
            "FATAL:".red().bold(),
            err.to_string().red()
        );
    }
}
