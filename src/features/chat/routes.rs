use actix_web::web;
use crate::features::chat::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("/{code}/messages", web::post().to(handlers::send_message))
            .route("/{code}/messages", web::get().to(handlers::get_messages)),
    );
}
