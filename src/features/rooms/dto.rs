use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//http response format
#[derive(Debug, Serialize, Deserialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub code: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<super::models::Room> for RoomResponse {
    fn from(room: super::models::Room) -> Self {
        Self {
            id: room.id,
            code: room.code,
            is_active: room.is_active,
            created_at: room.created_at,
        }
    }
}