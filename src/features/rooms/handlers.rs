use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::features::rooms::{
    dto::JoinRoomRequest,
    service::{RoomService, RoomServiceError},
};

#[derive(serde::Deserialize)]
pub struct LeaveRoomPath {
    pub code: String,
    pub participant_id: Uuid,
}


pub async fn create_room(pool: web::Data<PgPool>) -> impl Responder {
    match RoomService::create_room(&pool).await {
        Ok(room) => HttpResponse::Created().json(room),
        Err(RoomServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


pub async fn get_room(pool: web::Data<PgPool>,code: web::Path<String>) -> impl Responder {
    match RoomService::get_room(&pool, &code.into_inner()).await {
        Ok(Some(room)) => HttpResponse::Ok().json(room),
        Ok(None) => HttpResponse::NotFound().body("Room not found"),
        Err(RoomServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn join_room(
    pool: web::Data<PgPool>,
    code: web::Path<String>,
    body: web::Json<JoinRoomRequest>,
) -> impl Responder {
    match RoomService::join_room(&pool, &code.into_inner(), &body.nickname).await {
        Ok(participant) => HttpResponse::Created().json(participant),
        Err(RoomServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(RoomServiceError::InvalidNickname) => {
            HttpResponse::BadRequest().body("Nickname cannot be empty")
        }
        Err(RoomServiceError::NicknameAlreadyInUse) => {
            HttpResponse::Conflict().body("Nickname already in use in this room")
        }
        Err(RoomServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn leave_room(
    pool: web::Data<PgPool>,
    path: web::Path<LeaveRoomPath>,
) -> impl Responder {
    let path = path.into_inner();

    match RoomService::leave_room(&pool, &path.code, path.participant_id).await {
        Ok(participant) => HttpResponse::Ok().json(participant),
        Err(RoomServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(RoomServiceError::ParticipantNotFound) => {
            HttpResponse::NotFound().body("Participant not found or already left")
        }
        Err(RoomServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}