use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InitiateCallRequest {
    pub host_id: String,
}

#[derive(Debug, Serialize)]
pub struct CallResponse {
    pub id: Uuid,
    pub room_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}