use actix_web::web;
use crate::features::rooms::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/rooms", web::post().to(handlers::create_room))
        .route("/rooms/", web::post().to(handlers::create_room))
        .route("/rooms/{code}", web::get().to(handlers::get_room))
        .route("/rooms/{code}/join", web::post().to(handlers::join_room))
        .route(
            "/rooms/{code}/participants/{participant_id}/leave",
            web::post().to(handlers::leave_room),
        );
}