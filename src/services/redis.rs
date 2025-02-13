use ::std::time::{SystemTime, UNIX_EPOCH};
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TicketLock {
    user_id: String,
    locked_at: u64,
}

const TICKET_LOCK_PREFIX: &str = "ticket_lock:";

pub async fn lock_ticket(
    redis_client: &redis::Client,
    ticket_id: &str,
    user_id: &str,
    duration: u64,
) -> RedisResult<bool> {
    let mut conn = redis_client.get_async_connection().await?;
    let key = format!("{}{}", TICKET_LOCK_PREFIX, ticket_id);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let lock = TicketLock {
        user_id: user_id.to_string(),
        locked_at: now,
    };
    let lock_json = serde_json::to_string(&lock).unwrap();

    let set: bool = conn.set_ex(&key, lock_json, duration as usize).await?;
    Ok(set)
}

pub async fn check_ticket_lock(
    redis_client: &redis::Client,
    ticket_id: &str,
) -> RedisResult<Option<String>> {
    let mut conn = redis_client.get_async_connection().await?;
    let key = format!("{}{}", TICKET_LOCK_PREFIX, ticket_id);
    conn.get(key).await
}

pub async fn release_ticket(
    redis_client: &redis::Client,
    ticket_id: &str,
    user_id: &str,
) -> RedisResult<bool> {
    let mut conn = redis_client.get_async_connection().await?;
    let key = format!("{}{}", TICKET_LOCK_PREFIX, ticket_id);

    let lock: Option<String> = conn.get(&key).await?;
    if let Some(lock_data) = lock {
        let ticket: TicketLock = serde_json::from_str(&lock_data).unwrap();
        if ticket.user_id == user_id {
            let _: () = conn.del(&key).await?;
            return Ok(true);
        }
    }
    Ok(false)
}
