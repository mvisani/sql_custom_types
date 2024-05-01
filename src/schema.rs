// @generated automatically by Diesel CLI.

pub mod sql_types {
    use diesel::sql_types::Text;
    use diesel::{AsExpression, FromSqlRow};
    #[derive(
        Debug, FromSqlRow, AsExpression, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
    )]
    #[diesel(sql_type = Text)]
    #[diesel(postgres_type(name = "non_empty_text"))]
    pub struct NonEmptyText(pub String);
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
