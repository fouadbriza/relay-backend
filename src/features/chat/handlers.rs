use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::features::chat::{
    dto::{GetMessagesQuery, SendMessageRequest},
    service::{ChatService, ChatServiceError},
};

pub async fn send_message(
    pool: web::Data<PgPool>,
    code: web::Path<String>,
    body: web::Json<SendMessageRequest>,
) -> impl Responder {
    match ChatService::send_message(&pool, &code.into_inner(), body.participant_id, &body.content).await {
        Ok(message) => HttpResponse::Created().json(message),
        Err(ChatServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(ChatServiceError::ParticipantNotInRoom) => {
            HttpResponse::BadRequest().body("Participant is not active in this room")
        }
        Err(ChatServiceError::InvalidMessageContent) => {
            HttpResponse::BadRequest().body("Message content must be 1..2000 characters")
        }
        Err(ChatServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
    }
}

pub async fn get_messages(
    pool: web::Data<PgPool>,
    code: web::Path<String>,
    query: web::Query<GetMessagesQuery>,
) -> impl Responder {
    match ChatService::get_messages(&pool, &code.into_inner(), &query).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(ChatServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(ChatServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(ChatServiceError::InvalidMessageContent) => {
            HttpResponse::BadRequest().body("Invalid message content")
        }
        Err(ChatServiceError::ParticipantNotInRoom) => {
            HttpResponse::BadRequest().body("Participant is not active in this room")
        }
    }
}
