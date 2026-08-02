use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use shared::config::Config;
use shared::database::create_pool;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod features;
mod shared;

//routes
use crate::features::rooms::room_routes;
use crate::features::calls::call_routes;

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
    
    let db_pool = create_pool(&config.database_url)
    .await
    .expect("failed to connect to PostgreSQL");

    info!("Database connection pool established");
    
    info!("Running pending database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    info!("Database migrations complete");
    
    info!("Server starting on http://{}:{}", config.host, config.port);

    let pool_data = web::Data::new(db_pool);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool_data.clone())
            .service(health_check)
            .configure(call_routes)
            .configure(room_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}