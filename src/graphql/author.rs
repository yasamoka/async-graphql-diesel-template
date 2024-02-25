use async_graphql::{
    ComplexObject, Context, Object, Result as GraphQLResult, SimpleObject, Union, ID,
};
use diesel::{insert_into, pg::Pg, prelude::*};
use uuid::Uuid;

use crate::{
    connection::get_connection,
    graphql::book::{Book, DBBook},
    schema::{author, book},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = author)]
#[diesel(check_for_backend(Pg))]
pub struct DBAuthor {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Author {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
}

#[ComplexObject]
impl Author {
    async fn books(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<Book>> {
        let conn = &mut get_connection(ctx)?;
        let author_id = self.id.parse::<Uuid>().unwrap();
        let books = book::table
            .inner_join(author::table.on(book::author_id.eq(author::id)))
            .filter(author::id.eq(author_id))
            .select(DBBook::as_select())
            .load(conn)
            .map(|mut books| {
                books
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
                    .collect()
            })
            .unwrap();
        Ok(books)
    }
}

#[derive(Default)]
pub struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn author(&self, ctx: &Context<'_>, id: ID) -> GraphQLResult<Option<Author>> {
        let conn = &mut get_connection(ctx)?;
        let id = id.parse::<Uuid>().unwrap();
        let author = author::table
            .filter(author::id.eq(id))
            .select(DBAuthor::as_select())
            .first(conn)
            .optional()
            .unwrap()
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
            );
        Ok(author)
    }

    async fn authors(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<Author>> {
        let conn = &mut get_connection(ctx)?;
        let authors = author::table
            .select(DBAuthor::as_select())
            .load(conn)
            .unwrap()
            .drain(..)
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
            .collect();
        Ok(authors)
    }
}

#[derive(Union)]
enum AddAuthorResult {
    Success(AddAuthorSuccess),
}

#[derive(SimpleObject)]
struct AddAuthorSuccess {
    id: ID,
}

#[derive(Default)]
pub struct AuthorMutation;

#[Object]
impl AuthorMutation {
    async fn add_author(
        &self,
        ctx: &Context<'_>,
        first_name: String,
        last_name: String,
    ) -> GraphQLResult<AddAuthorResult> {
        #[derive(Insertable)]
        #[diesel(table_name = author)]
        #[diesel(check_for_backend(Pg))]
        struct NewAuthor {
            first_name: String,
            last_name: String,
        }

        #[derive(Selectable, Queryable)]
        #[diesel(table_name = author)]
        struct AuthorReturn {
            id: Uuid,
        }

        let conn = &mut get_connection(ctx)?;
        let new_author = NewAuthor {
            first_name,
            last_name,
        };
        let result = insert_into(author::table)
            .values(&new_author)
            .returning(AuthorReturn::as_returning())
            .get_result(conn)
            .map(|ret| AddAuthorResult::Success(AddAuthorSuccess { id: ret.id.into() }))?;
        Ok(result)
    }
}
