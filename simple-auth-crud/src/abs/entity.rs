use sqlx::{FromRow};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};

pub(crate) trait Entity<'r, T>: FromRow<'r, SqliteRow>
{
    fn primary_key(&self) -> &T;
    fn created_on(&self) -> &DateTime<Utc>;
}