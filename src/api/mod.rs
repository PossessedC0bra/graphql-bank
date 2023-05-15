use actix_web::web;

pub(crate) mod v5;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api").configure(v5::configure));
}
