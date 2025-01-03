use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, web, middleware::Logger};
use env_logger::Env;

mod handler;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(handler::index)
            .service(handler::new)
            .service(handler::show)
            .service(handler::create)
            .default_service(web::to(handler::not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8001")?.run().await
}
