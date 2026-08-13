use crate::features::rooms::models::{Participant, Room};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RoomRepository;

impl RoomRepository {

    pub async fn create(pool:&PgPool, code:&str) -> Result<Room,sqlx::Error> {
        sqlx::query_as::<_, Room>(
            r#"
            INSERT INTO rooms (code)
            VALUES ($1)
            RETURNING id, code, is_active, created_at, ended_at
            "#,
        )
        .bind(code)
        .fetch_one(pool)
        .await        
    }

    pub async fn find_active_by_code(pool: &PgPool, code: &str) -> Result<Option<Room>, sqlx::Error> {
        sqlx::query_as::<_, Room>(
            r#"
            SELECT id, code, is_active, created_at, ended_at
            FROM rooms
            WHERE code = $1 AND is_active = TRUE
            "#,
        )
        .bind(code)
        .fetch_optional(pool)
        .await
    }

    pub async fn create_participant(
        pool: &PgPool,
        room_id: Uuid,
        nickname: &str,
    ) -> Result<Participant, sqlx::Error> {
        sqlx::query_as::<_, Participant>(
            r#"
            INSERT INTO participants (room_id, nickname)
            VALUES ($1, $2)
            RETURNING id, room_id, nickname, joined_at, last_seen, left_at
            "#,
        )
        .bind(room_id)
        .bind(nickname)
        .fetch_one(pool)
        .await
    }

    pub async fn find_active_participant_by_room_and_nickname(
        pool: &PgPool,
        room_id: Uuid,
        nickname: &str,
    ) -> Result<Option<Participant>, sqlx::Error> {
        sqlx::query_as::<_, Participant>(
            r#"
            SELECT id, room_id, nickname, joined_at, last_seen, left_at
            FROM participants
            WHERE room_id = $1
            AND nickname = $2
            AND left_at IS NULL
            LIMIT 1
            "#,
        )
        .bind(room_id)
        .bind(nickname)
        .fetch_optional(pool)
        .await
    }

    pub async fn mark_participant_left_by_room_and_id(
        pool: &PgPool,
        room_id: Uuid,
        participant_id: Uuid,
    ) -> Result<Option<Participant>, sqlx::Error> {
        sqlx::query_as::<_, Participant>(
            r#"
            UPDATE participants
            SET left_at = NOW(), last_seen = NOW()
            WHERE room_id = $1
            AND id = $2
            AND left_at IS NULL
            RETURNING id, room_id, nickname, joined_at, last_seen, left_at
            "#,
        )
        .bind(room_id)
        .bind(participant_id)
        .fetch_optional(pool)
        .await
    }

}