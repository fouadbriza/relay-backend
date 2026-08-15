use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum ClientEvent {
    Join {
        participant_id: Uuid,
        nickname: String,
    },
    Message {
        participant_id: Uuid,
        content: String,
    },
    Leave {
        participant_id: Uuid,
    },
    Ping,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum ServerEvent {
    Joined {
        participant_id: Uuid,
        nickname: String,
    },
    UserJoined {
        participant_id: Uuid,
        nickname: String,
    },
    UserLeft {
        participant_id: Uuid,
    },
    MessageReceived {
        id: Uuid,
        room_code: String,
        participant_id: Uuid,
        content: String,
        created_at: DateTime<Utc>,
    },
    Error {
        message: String,
    },
    Pong,
}
