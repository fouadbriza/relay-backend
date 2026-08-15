use crate::features::chat::{
    dto::{GetMessagesQuery, MessageResponse},
    repository::ChatRepository,
};
use crate::features::rooms::repository::RoomRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum ChatServiceError {
    RoomNotFound,
    InvalidMessageContent,
    ParticipantNotInRoom,
    DatabaseError(sqlx::Error),
}

pub struct ChatService;

impl ChatService {
    pub async fn send_message(
        pool: &PgPool,
        room_code: &str,
        participant_id: Uuid,
        content: &str,
    ) -> Result<MessageResponse, ChatServiceError> {
        let trimmed = content.trim();
        if trimmed.is_empty() || trimmed.len() > 2000 {
            return Err(ChatServiceError::InvalidMessageContent);
        }

        let room = match RoomRepository::find_active_by_code(pool, room_code)
            .await
            .map_err(ChatServiceError::DatabaseError)?
        {
            Some(room) => room,
            None => return Err(ChatServiceError::RoomNotFound),
        };

        let participant = ChatRepository::find_active_participant_by_room_and_id(
            pool,
            room.id,
            participant_id,
        )
        .await
        .map_err(ChatServiceError::DatabaseError)?;

        if participant.is_none() {
            return Err(ChatServiceError::ParticipantNotInRoom);
        }

        let message = ChatRepository::create_message(pool, room.id, participant_id, trimmed)
            .await
            .map_err(ChatServiceError::DatabaseError)?;

        Ok(message.into())
    }

    pub async fn get_messages(
        pool: &PgPool,
        room_code: &str,
        query: &GetMessagesQuery,
    ) -> Result<Vec<MessageResponse>, ChatServiceError> {
        let room = match RoomRepository::find_active_by_code(pool, room_code)
            .await
            .map_err(ChatServiceError::DatabaseError)?
        {
            Some(room) => room,
            None => return Err(ChatServiceError::RoomNotFound),
        };

        let requested = query.limit.unwrap_or(100);
        let limit = requested.clamp(1, 100);

        let messages = ChatRepository::list_recent_messages_by_room(pool, room.id, limit)
            .await
            .map_err(ChatServiceError::DatabaseError)?;

        Ok(messages.into_iter().map(Into::into).collect())
    }
}
