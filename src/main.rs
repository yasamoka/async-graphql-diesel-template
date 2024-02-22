use async_graphql::http::GraphiQLSource;
use async_graphql_poem::GraphQL;
use poem::{handler, listener::TcpListener, post, web::Html, IntoResponse, Route, Server};

use async_graphql_diesel_template::graphql::build_schema;

const GRAPHQL_ENDPOINT: &str = "/graphql";

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint(GRAPHQL_ENDPOINT).finish())
}

#[tokio::main]
async fn main() {
    let schema = build_schema();
    let app = Route::new().at(GRAPHQL_ENDPOINT, post(GraphQL::new(schema)).get(graphiql));
    let listener = TcpListener::bind("localhost:3000");
    Server::new(listener).run(app).await.unwrap();
}
