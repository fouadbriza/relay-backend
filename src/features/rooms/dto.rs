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

#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub id: Uuid,
    pub room_id: Uuid,
    pub nickname: String,
    pub joined_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
}

impl From<super::models::Participant> for ParticipantResponse {
    fn from(participant: super::models::Participant) -> Self {
        Self {
            id: participant.id,
            room_id: participant.room_id,
            nickname: participant.nickname,
            joined_at: participant.joined_at,
            last_seen: participant.last_seen,
            left_at: participant.left_at,
        }
    }
}