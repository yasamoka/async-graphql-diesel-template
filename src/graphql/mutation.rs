use async_graphql::MergedObject;

use crate::graphql::{author::AuthorMutation, book::BookMutation};

#[derive(MergedObject, Default)]
pub struct Mutation(AuthorMutation, BookMutation);
