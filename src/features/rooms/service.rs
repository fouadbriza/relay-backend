use crate::features::rooms::{
    dto::{ParticipantResponse, RoomResponse},
    repository::RoomRepository,
};
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum RoomServiceError {
    RoomNotFound,
    InvalidNickname,
    NicknameAlreadyInUse,
    ParticipantNotFound,
    DatabaseError(sqlx::Error),
}

pub struct RoomService;

impl RoomService {
    pub async fn create_room(pool: &PgPool) -> Result<RoomResponse, RoomServiceError> {

        let code = generate_room_code();
        let room = RoomRepository::create(pool, &code)
            .await
            .map_err(RoomServiceError::DatabaseError)?;
        Ok(room.into())
    }

    pub async fn get_room(pool: &PgPool, code: &str) -> Result<Option<RoomResponse>, RoomServiceError> {
        let room_opt = RoomRepository::find_active_by_code(pool, code)
            .await
            .map_err(RoomServiceError::DatabaseError)?;
        Ok(room_opt.map(|room| room.into()))
    }

    pub async fn join_room(
        pool: &PgPool,
        code: &str,
        nickname: &str,
    ) -> Result<ParticipantResponse, RoomServiceError> {
        let nickname = nickname.trim();
        if nickname.is_empty() {
            return Err(RoomServiceError::InvalidNickname);
        }

        let room = match RoomRepository::find_active_by_code(pool, code)
            .await
            .map_err(RoomServiceError::DatabaseError)?
        {
            Some(room) => room,
            None => return Err(RoomServiceError::RoomNotFound),
        };

        let existing = RoomRepository::find_active_participant_by_room_and_nickname(
            pool,
            room.id,
            nickname,
        )
        .await
        .map_err(RoomServiceError::DatabaseError)?;

        if existing.is_some() {
            return Err(RoomServiceError::NicknameAlreadyInUse);
        }

        let participant = RoomRepository::create_participant(pool, room.id, nickname)
            .await
            .map_err(RoomServiceError::DatabaseError)?;

        Ok(participant.into())
    }

    pub async fn leave_room(
        pool: &PgPool,
        code: &str,
        participant_id: Uuid,
    ) -> Result<ParticipantResponse, RoomServiceError> {
        let room = match RoomRepository::find_active_by_code(pool, code)
            .await
            .map_err(RoomServiceError::DatabaseError)?
        {
            Some(room) => room,
            None => return Err(RoomServiceError::RoomNotFound),
        };

        let updated = RoomRepository::mark_participant_left_by_room_and_id(
            pool,
            room.id,
            participant_id,
        )
        .await
        .map_err(RoomServiceError::DatabaseError)?;

        match updated {
            Some(participant) => Ok(participant.into()),
            None => Err(RoomServiceError::ParticipantNotFound),
        }
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