use crate::features::calls::models::Call;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CallRepository;

impl CallRepository {
    pub async fn create(pool: &PgPool, room_id: Uuid, host_id: &str) -> Result<Call, sqlx::Error> {
        sqlx::query_as::<_, Call>(
            r#"
            INSERT INTO calls (room_id, host_id, status)
            VALUES ($1, $2, 'initiated')
            RETURNING id, room_id, host_id, status, created_at, ended_at
            "#,
        )
        .bind(room_id)
        .bind(host_id)
        .fetch_one(pool)
        .await
    }

    pub async fn find_active_by_room(pool: &PgPool, room_id: Uuid) -> Result<Option<Call>, sqlx::Error> {
        sqlx::query_as::<_, Call>(
            r#"
            SELECT id, room_id, host_id, status, created_at, ended_at
            FROM calls
            WHERE room_id = $1 AND status != 'ended'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(room_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn end_active_by_room(pool: &PgPool, room_id: Uuid) -> Result<Option<Call>, sqlx::Error> {
        sqlx::query_as::<_, Call>(
            r#"
            UPDATE calls
            SET status = 'ended', ended_at = NOW()
            WHERE room_id = $1 AND status != 'ended'
            RETURNING id, room_id, host_id, status, created_at, ended_at
            "#,
        )
        .bind(room_id)
        .fetch_optional(pool)
        .await
    }
}