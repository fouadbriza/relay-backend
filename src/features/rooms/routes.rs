use actix_web::web;
use crate::features::rooms::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("", web::post().to(handlers::create_room))
            .route("/{code}", web::get().to(handlers::get_room)),
    );
}