use async_graphql::{EmptySubscription, Schema};

use crate::graphql::{mutation::Mutation, query::Query};

pub fn build_schema() -> Schema<Query, Mutation, EmptySubscription> {
    Schema::new(Query::default(), Mutation::default(), EmptySubscription)
}
