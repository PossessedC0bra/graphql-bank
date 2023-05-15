use juniper::Context;
use sqlx::AnyPool;

pub struct GraphQlContext {
    pub connection_pool: AnyPool,
}

impl Context for GraphQlContext {}
