use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub participant_id: Uuid,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub room_id: Uuid,
    pub participant_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<super::models::Message> for MessageResponse {
    fn from(message: super::models::Message) -> Self {
        Self {
            id: message.id,
            room_id: message.room_id,
            participant_id: message.participant_id,
            content: message.content,
            created_at: message.created_at,
        }
    }
}
