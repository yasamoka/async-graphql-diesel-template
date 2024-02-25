use async_graphql::{
    ComplexObject, Context, Enum as GraphQLEnum, Object, Result as GraphQLResult, SimpleObject,
    Union, ID,
};
use diesel::{dsl::exists, insert_into, pg::Pg, prelude::*, select};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

use crate::{
    connection::get_connection,
    graphql::author::{Author, DBAuthor},
    schema::{author, book, sql_types::Binding as BindingPGEnum},
};

use super::error::AuthorNotFoundError;

#[derive(Copy, Clone, Eq, PartialEq, Debug, GraphQLEnum, DbEnum)]
#[ExistingTypePath = "BindingPGEnum"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum Binding {
    Hardcover,
    Paperback,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = book)]
#[diesel(check_for_backend(Pg))]
pub struct DBBook {
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub description: Option<String>,
    pub binding: Binding,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Book {
    #[graphql(skip)]
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub description: Option<String>,
    pub binding: Binding,
}

#[ComplexObject]
impl Book {
    async fn author(&self, ctx: &Context<'_>) -> async_graphql::Result<Author> {
        let conn = &mut get_connection(ctx)?;
        let author = author::table
            .inner_join(book::table.on(author::id.eq(book::author_id)))
            .filter(book::id.eq(self.id))
            .select(DBAuthor::as_select())
            .first(conn)
            .map(
                |DBAuthor {
                     id,
                     first_name,
                     last_name,
                 }| Author {
                    id: id.into(),
                    first_name,
                    last_name,
                },
            )
            .unwrap();
        Ok(author)
    }
}

#[derive(Default)]
pub struct BookQuery;

#[Object]
impl BookQuery {
    async fn book(&self, ctx: &Context<'_>, isbn: String) -> GraphQLResult<Option<Book>> {
        let conn = &mut get_connection(ctx)?;
        let book = book::table
            .filter(book::isbn.eq(isbn))
            .select(DBBook::as_select())
            .first(conn)
            .optional()
            .unwrap()
            .map(
                |DBBook {
                     id,
                     title,
                     isbn,
                     description,
                     binding,
                 }| Book {
                    id,
                    isbn,
                    title,
                    description,
                    binding,
                },
            );
        Ok(book)
    }

    async fn books(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<Book>> {
        let conn = &mut get_connection(ctx)?;
        let books = book::table
            .select(DBBook::as_select())
            .load(conn)
            .unwrap()
            .drain(..)
            .map(
                |DBBook {
                     id,
                     isbn,
                     title,
                     description,
                     binding,
                 }| Book {
                    id,
                    isbn,
                    title,
                    description,
                    binding,
                },
            )
            .collect();
        Ok(books)
    }
}

#[derive(Union)]
enum AddBookResult {
    Success(AddBookSuccess),
    AuthorNotFound(AuthorNotFoundError),
}

#[derive(Selectable, Queryable, SimpleObject)]
#[diesel(table_name = book)]
#[diesel(check_for_backend(Pg))]
struct AddBookSuccess {
    id: i32,
}

#[derive(Default)]
pub struct BookMutation;

#[Object]
impl BookMutation {
    async fn add_book(
        &self,
        ctx: &Context<'_>,
        author_id: ID,
        isbn: String,
        title: String,
        description: Option<String>,
        binding: Binding,
    ) -> async_graphql::Result<AddBookResult> {
        #[derive(Insertable)]
        #[diesel(table_name = book)]
        #[diesel(check_for_backend(Pg))]
        struct NewBook {
            author_id: Uuid,
            isbn: String,
            title: String,
            description: Option<String>,
            binding: Binding,
        }

        let conn = &mut get_connection(ctx)?;

        let author_id = author_id.parse::<Uuid>().unwrap();
        let author_exists = select(exists(author::table.filter(author::id.eq(author_id))))
            .get_result::<bool>(conn)?;
        match author_exists {
            true => {
                let new_book = NewBook {
                    author_id,
                    isbn,
                    title,
                    description,
                    binding,
                };

                let result = insert_into(book::table)
                    .values(&new_book)
                    .returning(AddBookSuccess::as_returning())
                    .get_result(conn)
                    .map(|success| AddBookResult::Success(success))?;

                Ok(result)
            }
            false => Ok(AddBookResult::AuthorNotFound(AuthorNotFoundError {
                id: author_id.into(),
            })),
        }
    }
}
