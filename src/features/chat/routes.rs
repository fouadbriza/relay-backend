use actix_web::web;
use crate::features::chat::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/rooms/{code}/messages", web::post().to(handlers::send_message))
        .route("/rooms/{code}/messages", web::get().to(handlers::get_messages))
        .route("/rooms/{code}/ws", web::get().to(handlers::ws_connect));
}
