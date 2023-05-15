use actix_web::web;
use actix_web::web::Data;

use crate::graphql::schema;

pub(crate) mod graphql_resource;

pub fn configure(config: &mut web::ServiceConfig) {
    let schema = schema::create_schema();

    config.service(
        web::scope("/v5")
            .app_data(Data::new(schema))
            .service(graphql_resource::graphql)
            .service(graphql_resource::graphql_playground),
    );
}
