use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use crate::shared::config::Config;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod features;
mod shared;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("server is working")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Server starting on http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}