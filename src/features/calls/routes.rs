use actix_web::web;
use crate::features::calls::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/rooms/{code}/calls", web::post().to(handlers::initiate_call))
        .route("/rooms/{code}/calls/end", web::post().to(handlers::end_call));
}