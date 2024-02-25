use async_graphql::{EmptySubscription, Schema};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::graphql::{mutation::Mutation, query::Query};

pub fn build_schema(
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(connection_pool)
        .finish()
}
