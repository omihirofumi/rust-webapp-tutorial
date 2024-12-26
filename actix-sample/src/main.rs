use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get};
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8001")?.run().await
}
