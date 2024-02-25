// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "binding"))]
    pub struct Binding;
}

diesel::table! {
    author (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Binding;

    book (id) {
        id -> Int4,
        author_id -> Uuid,
        isbn -> Text,
        title -> Text,
        description -> Nullable<Text>,
        binding -> Binding,
    }
}

diesel::joinable!(book -> author (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    book,
);
