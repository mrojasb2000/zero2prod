use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

//noinspection RsMainFunctionNotFound
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
