use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: Uuid,
    pub code: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Participant {
    pub id: Uuid,
    pub room_id: Uuid,
    pub nickname: String,
    pub joined_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
}