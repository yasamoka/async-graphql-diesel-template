use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct DBError {
    pub message: String,
}

impl From<diesel::result::Error> for DBError {
    fn from(value: diesel::result::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<DBError> for async_graphql::Error {
    fn from(value: DBError) -> Self {
        async_graphql::Error {
            message: value.message,
            source: None,
            extensions: None,
        }
    }
}

#[derive(SimpleObject)]
pub struct AuthorNotFoundError {
    pub id: i32,
}
