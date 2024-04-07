use actix_web::{App, HttpServer};

mod controller;
mod model;
mod service;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().configure(controller::fund_config))
        .bind("0.0.0.0:8099")?
        .run()
        .await
}
