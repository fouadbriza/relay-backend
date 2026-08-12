use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::features::calls::dto::{CallResponse, InitiateCallRequest};
use crate::features::calls::service::{CallService, CallServiceError};

pub async fn initiate_call(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    req: web::Json<InitiateCallRequest>,
) -> impl Responder {
    let room_code = path.into_inner();

    match CallService::initiate_call(&pool, &room_code, &req.host_id).await {
        Ok(call) => HttpResponse::Created().json(CallResponse {
            id: call.id,
            room_id: call.room_id,
            status: call.status,
            created_at: call.created_at,
            ended_at: call.ended_at,
        }),
        Err(CallServiceError::RoomNotFoundOrInactive) => {
            HttpResponse::BadRequest().body("Room not found or inactive")
        }
        Err(CallServiceError::DatabaseError(_)) => {
            HttpResponse::InternalServerError().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn end_call(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let room_code = path.into_inner();

    match CallService::end_call(&pool, &room_code).await {
        Ok(call) => HttpResponse::Ok().json(CallResponse {
            id: call.id,
            room_id: call.room_id,
            status: call.status,
            created_at: call.created_at,
            ended_at: call.ended_at,
        }),
        Err(CallServiceError::RoomNotFoundOrInactive) => {
            HttpResponse::BadRequest().body("Room not found or inactive")
        }
        Err(CallServiceError::NoActiveCall) => {
            HttpResponse::NotFound().body("No active call to end in this room")
        }
        Err(CallServiceError::DatabaseError(_)) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}