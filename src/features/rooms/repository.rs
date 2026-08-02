use crate::features::rooms::models::Room;
use sqlx::PgPool;

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

}