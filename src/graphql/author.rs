use async_graphql::{ComplexObject, Object, SimpleObject, Union, ID};
use diesel::{insert_into, pg::Pg, prelude::*};
use uuid::Uuid;

use crate::{
    database::establish_connection,
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
    async fn books(&self) -> Vec<Book> {
        let conn = &mut establish_connection();
        let author_id = self.id.parse::<Uuid>().unwrap();
        book::table
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
            .unwrap()
    }
}

#[derive(Default)]
pub struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn author(&self, id: ID) -> Option<Author> {
        let conn = &mut establish_connection();
        let id = id.parse::<Uuid>().unwrap();
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
                    id: id.into(),
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
                    id: id.into(),
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

        #[derive(Selectable, Queryable)]
        #[diesel(table_name = author)]
        struct AuthorReturn {
            id: Uuid,
        }

        let conn = &mut establish_connection();
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
