use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::models::ticket::{
    LockTicketRequest, LockedTicketResponse, ReleaseTicketRequest, TicketLockData,
};
use crate::services::redis::{
    check_ticket_lock, get_all_locked_tickets, lock_ticket, release_ticket,
};
use redis::Client;

pub fn ticket_routes(redis_client: Arc<Client>) -> Router {
    Router::new()
        .route("/lock", post(lock_ticket_handler))
        .route("/check/:ticket_id", get(check_ticket_handler))
        .route("/release", post(release_ticket_handler))
        .route("/locked-tickets", get(get_all_locked_tickets_handler))
        .with_state(redis_client)
}

async fn lock_ticket_handler(
    State(redis_client): State<Arc<Client>>,
    Json(payload): Json<LockTicketRequest>,
) -> Json<LockedTicketResponse<String>> {
    let result = lock_ticket(
        &redis_client,
        &payload.ticket_id,
        &payload.user_id,
        payload.duration,
    )
    .await;
    match result {
        Ok(true) => Json(LockedTicketResponse {
            message: "Ticket locked successfully".to_string(),
            data: vec![payload.ticket_id],
        }),
        _ => Json(LockedTicketResponse {
            message: "Failed to lock ticket".to_string(),
            data: vec![],
        }),
    }
}

/// Handler to check if a ticket is locked
///
/// This handler checks if a ticket is locked by checking if a lock exists in Redis.
///
/// # Returns
///
/// The JSON response containing the message and data.
///
/// # Example
///
/// ```json
/// {
///  "message": "Ticket is locked",
/// "data": [
///  {
///   "user_id": "user_id",
///  "locked_at": 1234567890
///  }
///  ]
/// }
/// ```
async fn check_ticket_handler(
    State(redis_client): State<Arc<Client>>,
    Path(ticket_id): Path<String>,
) -> Json<LockedTicketResponse<TicketLockData>> {
    let result = check_ticket_lock(&redis_client, &ticket_id).await;

    match result {
        Ok(Some(lock_data)) => {
            if let Ok(ticket_lock) = serde_json::from_str::<TicketLockData>(&lock_data) {
                Json(LockedTicketResponse {
                    message: "Ticket is locked".to_string(),
                    data: vec![ticket_lock], // Return the struct directly
                })
            } else {
                Json(LockedTicketResponse {
                    message: "Failed to parse locked ticket data".to_string(),
                    data: vec![],
                })
            }
        }
        _ => Json(LockedTicketResponse {
            message: "Ticket is available".to_string(),
            data: vec![],
        }),
    }
}

/// Handler to release a locked ticket
///
/// This handler releases a locked ticket by removing the lock from Redis.
///
/// # Returns
///
/// The JSON response containing the message and data.
///
/// # Example
///
/// ```json
/// {
///   "message": "Ticket released",
///  "data": [
///    "ticket_id"
///   ]
/// }
/// ```
async fn release_ticket_handler(
    State(redis_client): State<Arc<Client>>,
    Json(payload): Json<ReleaseTicketRequest>,
) -> Json<LockedTicketResponse<String>> {
    let result = release_ticket(&redis_client, &payload.ticket_id, &payload.user_id).await;
    match result {
        Ok(true) => Json(LockedTicketResponse {
            message: "Ticket released".to_string(),
            data: vec![payload.ticket_id],
        }),
        _ => Json(LockedTicketResponse {
            message: "Ticket isn't locked or failed to release ticket".to_string(),
            data: vec![payload.ticket_id],
        }),
    }
}

/// Handler to get all locked tickets
///
/// This handler retrieves all locked tickets from Redis and returns them as a JSON response.
///
/// # Returns
///
/// The JSON response containing the message and data.
///
/// # Example
///
/// ```json
/// {
///    "message": "Locked tickets retrieved",
///   "data": [
///      "ticket_id_1",
///     "ticket_id_2"
///    ]
/// }
/// ```
async fn get_all_locked_tickets_handler(
    State(redis_client): State<Arc<Client>>,
) -> Json<LockedTicketResponse<String>> {
    let result = get_all_locked_tickets(&redis_client).await;
    match result {
        Ok(tickets) => Json(LockedTicketResponse {
            message: "Locked tickets retrieved".to_string(),
            data: tickets,
        }),
        Err(_) => Json(LockedTicketResponse {
            message: "Failed to retrieve locked tickets".to_string(),
            data: vec![],
        }),
    }
}
