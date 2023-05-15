use crate::graphql::{graphql_context::GraphQlContext, schema::Schema};

use actix_web::{get, route, web, Error, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use sqlx::AnyPool;

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    pool: web::Data<AnyPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQlContext {
        connection_pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphql/playground")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/api/v5/graphql", None))
}
