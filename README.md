# SQL Custom Types (an their conversion to Rust)

We discovered that we could create [custom types in SQL databases](https://www.postgresql.org/docs/current/sql-createtype.html). This would then allow us to check inputs coming from the frontend and ensure that they are the correct types. As a first example we tried to create a type `non_empty_text` where we have some text but the string should not be emtpy. 

According to Postgres documentation, the most straightforward way to do this is to create a domain. A domain is a data type with optional constraints.
```sql
CREATE DOMAIN non_empty_text AS TEXT CHECK (VALUE <> '');
```

However [Diesel](https://diesel.rs/) detected this as `Text` and not as `non_empty_text`. This causes a problems since we wanted the *our* custom type in Rust and not `diesel::sql_types::Text`.

In Postgres, the composite type is a type specified by a list of attribute names and data types. We then tried to create a custom composite type where the attribute is a domain:
```sql
CREATE DOMAIN non_empty_text_in AS TEXT
    CHECK (value <> '');

CREATE type non_empty_text AS (
    value non_empty_text_in
);
```

Now [Diesel](https://diesel.rs/) detects it as a custom type (as seen in `schema.rs`): 
```rust
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
```

Now that we have created a new custom SQL type called `NonEmptyText`, we need to create the conversion from SQL to Rust:
```rust
#[derive(Debug, FromSqlRow, AsExpression, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[diesel(sql_type = crate::schema::sql_types::NonEmptyText)]
pub struct NonEmptyTextRust(String);
```

Following [this tutorial](https://kitsu.me/posts/2020_05_24_custom_types_in_diesel) we implemented the `FromSql` and `ToSql` traits for the custom type `NonEmptyTextRust`. 

For the `ToSql` trait we used the helper trait `WriteTuple` from Diesel for writing tuples as named composite types. 