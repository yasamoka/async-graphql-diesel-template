use async_graphql::{ComplexObject, Object, SimpleObject, Union};
use diesel::{insert_into, pg::Pg, prelude::*};

use crate::{
    database::establish_connection,
    graphql::book::{Book, DBBook},
    schema::{author, book},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = author)]
#[diesel(check_for_backend(Pg))]
pub struct DBAuthor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[ComplexObject]
impl Author {
    async fn books(&self) -> Vec<Book> {
        let conn = &mut establish_connection();
        book::table
            .inner_join(author::table.on(book::author_id.eq(author::id)))
            .filter(author::id.eq(self.id))
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
            .unwrap()
    }
}

#[derive(Default)]
pub struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn author(&self, id: i32) -> Option<Author> {
        let conn = &mut establish_connection();
        author::table
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
                    id,
                    first_name,
                    last_name,
                },
            )
    }

    async fn authors(&self) -> Vec<Author> {
        let conn = &mut establish_connection();
        author::table
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
                    id,
                    first_name,
                    last_name,
                },
            )
            .collect()
    }
}

#[derive(Union)]
enum AddAuthorResult {
    Success(AddAuthorSuccess),
}

#[derive(Selectable, Queryable, SimpleObject)]
#[diesel(table_name = author)]
#[diesel(check_for_backend(Pg))]
struct AddAuthorSuccess {
    id: i32,
}

#[derive(Default)]
pub struct AuthorMutation;

#[Object]
impl AuthorMutation {
    async fn add_author(
        &self,
        first_name: String,
        last_name: String,
    ) -> async_graphql::Result<AddAuthorResult> {
        #[derive(Insertable)]
        #[diesel(table_name = author)]
        #[diesel(check_for_backend(Pg))]
        struct NewAuthor {
            first_name: String,
            last_name: String,
        }

        let conn = &mut establish_connection();
        let new_author = NewAuthor {
            first_name,
            last_name,
        };
        let result = insert_into(author::table)
            .values(&new_author)
            .returning(AddAuthorSuccess::as_returning())
            .get_result(conn)
            .map(|success| AddAuthorResult::Success(success))?;
        Ok(result)
    }
}
