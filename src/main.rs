use actix_web::{App, HttpServer};

mod controller;
mod model;
mod service;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().configure(controller::fund_config))
        .bind("127.0.0.1:8099")?
        .run()
        .await
}
