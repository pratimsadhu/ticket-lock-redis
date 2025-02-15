use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TicketLockData {
    pub user_id: String,
    pub locked_at: u64,
}

#[derive(Deserialize)]
pub struct LockTicketRequest {
    pub ticket_id: String,
    pub user_id: String,
    pub duration: u64,
}

#[derive(Deserialize)]
pub struct ReleaseTicketRequest {
    pub ticket_id: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct LockedTicketResponse<T> {
    pub message: String,
    pub data: Vec<T>,
}
