use redis::Client;
use std::env;

pub fn get_redis_client() -> Client {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    Client::open(redis_url).expect("Failed to create Redis Client")
}
