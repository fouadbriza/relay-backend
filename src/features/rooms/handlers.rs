use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::features::rooms::{
    service::RoomService,
};


pub async fn create_room(pool: web::Data<PgPool>) -> impl Responder {
    match RoomService::create_room(&pool).await {
        Ok(room) => HttpResponse::Created().json(room),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}


pub async fn get_room(pool: web::Data<PgPool>,code: web::Path<String>) -> impl Responder {
    match RoomService::get_room(&pool, &code.into_inner()).await {
        Ok(Some(room)) => HttpResponse::Ok().json(room),
        Ok(None) => HttpResponse::NotFound().body("Room not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}