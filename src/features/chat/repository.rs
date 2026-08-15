use crate::features::chat::models::Message;
use crate::features::rooms::models::Participant;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ChatRepository;

impl ChatRepository {
    pub async fn find_active_participant_by_room_and_id(
        pool: &PgPool,
        room_id: Uuid,
        participant_id: Uuid,
    ) -> Result<Option<Participant>, sqlx::Error> {
        sqlx::query_as::<_, Participant>(
            r#"
            SELECT id, room_id, nickname, joined_at, last_seen, left_at
            FROM participants
            WHERE room_id = $1
              AND id = $2
              AND left_at IS NULL
            LIMIT 1
            "#,
        )
        .bind(room_id)
        .bind(participant_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create_message(
        pool: &PgPool,
        room_id: Uuid,
        participant_id: Uuid,
        content: &str,
    ) -> Result<Message, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (room_id, participant_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, room_id, participant_id, content, created_at
            "#,
        )
        .bind(room_id)
        .bind(participant_id)
        .bind(content)
        .fetch_one(pool)
        .await
    }

    pub async fn list_recent_messages_by_room(
        pool: &PgPool,
        room_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            r#"
            SELECT id, room_id, participant_id, content, created_at
            FROM messages
            WHERE room_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(room_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}
