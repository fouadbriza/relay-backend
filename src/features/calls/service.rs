use sqlx::PgPool;

use crate::features::calls::models::Call;
use crate::features::calls::repository::CallRepository;
use crate::features::rooms::repository::RoomRepository;

#[derive(Debug)]
pub enum CallServiceError {
    RoomNotFoundOrInactive,
    NoActiveCall,
    DatabaseError(sqlx::Error),
}

pub struct CallService;

impl CallService {
    pub async fn initiate_call(
        pool: &PgPool,
        room_code: &str,
        host_id: &str,
    ) -> Result<Call, CallServiceError> {
        let room = match RoomRepository::find_active_by_code(pool, room_code).await {
            Ok(Some(room)) => room,
            Ok(None) => return Err(CallServiceError::RoomNotFoundOrInactive),
            Err(e) => return Err(CallServiceError::DatabaseError(e)),
        };

        CallRepository::create(pool, room.id, host_id)
            .await
            .map_err(CallServiceError::DatabaseError)
    }

    pub async fn end_call(
        pool: &PgPool,
        room_code: &str,
    ) -> Result<Call, CallServiceError> {
        let room = match RoomRepository::find_active_by_code(pool, room_code).await {
            Ok(Some(room)) => room,
            Ok(None) => return Err(CallServiceError::RoomNotFoundOrInactive),
            Err(e) => return Err(CallServiceError::DatabaseError(e)),
        };

        match CallRepository::end_active_by_room(pool, room.id).await {
            Ok(Some(call)) => Ok(call),
            Ok(None) => Err(CallServiceError::NoActiveCall),
            Err(e) => Err(CallServiceError::DatabaseError(e)),
        }
    }
}