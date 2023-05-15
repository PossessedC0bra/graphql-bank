use juniper::{EmptySubscription, RootNode};

use super::graphql_context::GraphQlContext;

use self::{mutation::Mutation, query::Query};

pub(crate) mod mutation;
pub(crate) mod query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQlContext>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        Mutation {},
        EmptySubscription::<GraphQlContext>::new(),
    )
}
