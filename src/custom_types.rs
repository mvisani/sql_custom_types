use diesel::pg::Pg;
use diesel::serialize::Output;
use diesel::serialize::WriteTuple;
use diesel::sql_types::Text;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
};

#[derive(Debug, FromSqlRow, AsExpression, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[diesel(sql_type = crate::schema::sql_types::NonEmptyText)]
pub struct NonEmptyTextRust(String);

impl diesel::serialize::ToSql<crate::schema::sql_types::NonEmptyText, Pg> for NonEmptyTextRust {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        WriteTuple::<(Text,)>::write_tuple(&(self.0.clone(),), &mut out.reborrow())
    }
}

impl FromSql<crate::schema::sql_types::NonEmptyText, diesel::pg::Pg> for NonEmptyTextRust {
    fn from_sql(
        bytes: <diesel::pg::Pg as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let (string,) = <(String,)>::from_sql(bytes)?;
        if string.is_empty() {
            return Err("NonEmptyText cannot be empty".into());
        }
        Ok(NonEmptyTextRust(string))
    }
}

impl From<&str> for NonEmptyTextRust {
    fn from(s: &str) -> Self {
        NonEmptyTextRust(s.into())
    }
}
