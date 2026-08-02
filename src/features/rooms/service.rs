use crate::features::rooms::{
    dto::RoomResponse,
    repository::RoomRepository,
};
use rand::Rng;
use sqlx::PgPool;

pub struct RoomService;

impl RoomService {
    pub async fn create_room(pool: &PgPool) -> Result<RoomResponse, sqlx::Error> {

        let code = generate_room_code();
        let room = RoomRepository::create(pool, &code).await?;
        Ok(room.into())
    }

    pub async fn get_room(pool: &PgPool, code: &str) -> Result<Option<RoomResponse>, sqlx::Error> {
        let room_opt = RoomRepository::find_active_by_code(pool, code).await?;     
        Ok(room_opt.map(|room| room.into()))
    }
}


fn generate_room_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();

    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}