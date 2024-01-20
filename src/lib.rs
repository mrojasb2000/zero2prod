//! lib.rs
use actix_web::{web, App, HttpServer, HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//noinspection RsMainFunctionNotFound
pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}