// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "non_empty_text"))]
    pub struct NonEmptyText;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NonEmptyText;

    users (id) {
        id -> Int4,
        first_name -> NonEmptyText,
        middle_name -> Nullable<NonEmptyText>,
        last_name -> NonEmptyText,
    }
}
