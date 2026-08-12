use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::calls::models::Call;

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

impl From<Call> for CallResponse {
    fn from(call: Call) -> Self {
        Self {
            id: call.id,
            room_id: call.room_id,
            status: call.status,
            created_at: call.created_at,
            ended_at: call.ended_at,
        }
    }
}