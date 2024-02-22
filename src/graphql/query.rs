use async_graphql::MergedObject;

use crate::graphql::{author::AuthorQuery, book::BookQuery};

#[derive(MergedObject, Default)]
pub struct Query(AuthorQuery, BookQuery);
