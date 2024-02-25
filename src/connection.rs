use async_graphql::Context;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub fn get_connection(
    ctx: &Context<'_>,
) -> async_graphql::Result<PooledConnection<ConnectionManager<PgConnection>>> {
    let pool = ctx.data::<Pool<ConnectionManager<PgConnection>>>().unwrap();
    pool.get().map_err(|err| async_graphql::Error {
        message: err.to_string(),
        source: None,
        extensions: None,
    })
}
