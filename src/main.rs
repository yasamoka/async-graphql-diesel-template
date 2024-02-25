use std::env;

use async_graphql::http::GraphiQLSource;
use async_graphql_diesel_template::graphql::build_schema;
use async_graphql_poem::GraphQL;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use poem::{handler, listener::TcpListener, post, web::Html, IntoResponse, Route, Server};

const GRAPHQL_ENDPOINT: &str = "/graphql";

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint(GRAPHQL_ENDPOINT).finish())
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[tokio::main]
async fn main() {
    let connection_pool = get_connection_pool();
    let schema = build_schema(connection_pool);
    let app = Route::new().at(GRAPHQL_ENDPOINT, post(GraphQL::new(schema)).get(graphiql));
    let listener = TcpListener::bind("localhost:3000");
    Server::new(listener).run(app).await.unwrap();
}
