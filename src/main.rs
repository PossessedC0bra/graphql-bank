use actix_web::{middleware, web::Data, App, HttpServer};
use dotenvy::dotenv;
use persistence::connection::create_connection_pool;
use std::io::Result;

pub(crate) mod api;
pub(crate) mod error;
pub(crate) mod graphql;
pub(crate) mod persistence;
pub(crate) mod service;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    let pool = create_connection_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(api::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
