use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::features::calls::dto::{CallResponse, InitiateCallRequest};
use crate::features::calls::repository::CallRepository;
use crate::features::rooms::repository::RoomRepository;

pub async fn initiate_call(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    req: web::Json<InitiateCallRequest>,
) -> impl Responder {
    let room_code = path.into_inner();

    let room = match RoomRepository::find_active_by_code(&pool, &room_code).await {
        Ok(Some(room)) => room,
        Ok(_) => return HttpResponse::BadRequest().body("Room not found or inactive"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match CallRepository::create(&pool, room.id, &req.host_id).await {
        Ok(call) => HttpResponse::Created().json(CallResponse {
            id: call.id,
            room_id: call.room_id,
            status: call.status,
            created_at: call.created_at,
            ended_at: call.ended_at,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn end_call(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let room_code = path.into_inner();


    let room = match RoomRepository::find_active_by_code(&pool, &room_code).await {
        Ok(Some(room)) => room,
        Ok(_) => return HttpResponse::BadRequest().body("Room not found or inactive"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    
    match CallRepository::end_active_by_room(&pool, room.id).await {
        Ok(Some(call)) => HttpResponse::Ok().json(CallResponse {
            id: call.id,
            room_id: call.room_id,
            status: call.status,
            created_at: call.created_at,
            ended_at: call.ended_at,
        }),
        Ok(None) => HttpResponse::NotFound().body("No active call to end in this room"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}