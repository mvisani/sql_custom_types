// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::sql_types::NonEmptyText;
use crate::schema::users;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: NonEmptyText,
    pub middle_name: Option<NonEmptyText>,
    pub last_name: NonEmptyText,
}

// #[derive(Debug, FromSqlRow, AsExpression)]
// #[diesel(sql_type = crate::schema::sql_types::NonEmptyText)]
// pub struct NonEmptyString(pub String);

impl User {
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
    }
}