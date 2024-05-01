use crate::schema::sql_types::NonEmptyText;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text; // Import PostgreSQL specific types if you're using PostgreSQL
use diesel::AsExpression;
use diesel::FromSqlRow;

// // Implement necessary traits for the new SQL type
// impl<Pg> ToSql<Text, Pg> for NonEmptyText
// where
//     Pg: Backend,
//     String: ToSql<Text, Pg>,
// {
//     fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
//         self.0.to_sql(out)
//     }
// }

// impl<Pg> FromSql<Text, Pg> for NonEmptyText
// where
//     Pg: Backend,
//     String: FromSql<Text, Pg>,
// {
//     fn from_sql(bytes: Pg::RawValue) -> deserialize::Result<Self> {
//         let s = String::from_sql(bytes)?;
//         if s.is_empty() {
//             Err("Empty strings are not allowed".into())
//         } else {
//             Ok(NonEmptyText(s))
//         }
//     }
// }
