use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::shared::config::Config;

mod features;
mod shared;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("server is working")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    println!("server running on http://{}:{}",config.host,config.port);
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}