use actix_web::web;
use crate::features::rooms::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("", web::post().to(handlers::create_room))
            .route("/{code}", web::get().to(handlers::get_room)),
    )
    .service(
        web::scope("/rooms")
            .route("/{code}/join", web::post().to(handlers::join_room))
            .route(
                "/{code}/participants/{participant_id}/leave",
                web::post().to(handlers::leave_room),
            ),
    );
}