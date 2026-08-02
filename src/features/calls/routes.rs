use actix_web::web;
use crate::features::calls::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms/{code}/calls")
            .route("", web::post().to(handlers::initiate_call))
            .route("/end", web::post().to(handlers::end_call)),
    );
}