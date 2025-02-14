use redis::Client;
use std::env;
use colored::*;
use thiserror::Error;

/// Possible errors that can occur when connecting to Redis
#[derive(Error, Debug)]
pub enum RedisConnectionError {
    #[error("Environment variable REDIS_URL is not set or invalid")]
    EnvVarError(#[from] env::VarError),
    
    #[error("Failed to create Redis client: {0}")]
    ClientCreationError(String),
    
    #[error("Failed to connect to Redis server: {0}")]
    ConnectionError(String),
    
    #[error("Redis server is not responding: {0}")]
    ServerNotResponding(String),
}

/// Configuration for Redis connection
#[derive(Debug)]
pub struct RedisConfig {
    url: String,
    default_port: u16,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost".to_string(),
            default_port: 6379,
        }
    }
}

/// Attempts to establish a connection to Redis and returns a client if successful
///
/// # Returns
/// - `Ok(Client)` if the connection is successful
/// - `Err(RedisConnectionError)` if any step of the connection process fails
///
/// # Example
/// ```rust
/// match get_redis_client() {
///     Ok(client) => println!("Successfully connected to Redis"),
///     Err(err) => eprintln!("Failed to connect: {}", err),
/// }
/// ```
pub fn get_redis_client() -> Result<Client, RedisConnectionError> {
    let config = RedisConfig::default();
    
    // Get Redis URL from environment variable
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| format!("{}:{}", config.url, config.default_port));

    // Create Redis client
    let client = Client::open(redis_url.clone()).map_err(|err| {
        log_error("Failed to create Redis client", &err.to_string());
        RedisConnectionError::ClientCreationError(err.to_string())
    })?;

    // Test connection by getting a connection instance
    let mut conn = client.get_connection().map_err(|err| {
        log_error(
            "Failed to connect to Redis server",
            &format!("{}. Please ensure Redis is running on port {} or update REDIS_URL", 
                err, config.default_port)
        );
        RedisConnectionError::ConnectionError(err.to_string())
    })?;

    // Verify connection with PING command
    redis::cmd("PING").query::<String>(&mut conn).map_err(|err| {
        log_error(
            "Redis server is not responding",
            &format!("{}. Please check if Redis server is running properly", err)
        );
        RedisConnectionError::ServerNotResponding(err.to_string())
    })?;

    // Log successful connection
    println!("{} Successfully connected to Redis at {}", 
        "INFO:".green().bold(), 
        redis_url.cyan()
    );

    Ok(client)
}

/// Helper function to log errors in a consistent format
fn log_error(context: &str, details: &str) {
    eprintln!("{} {}: {}",
        "ERROR:".red().bold(),
        context.bold(),
        details.red()
    );
}