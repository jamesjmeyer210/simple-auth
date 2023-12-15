use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};
use sqlx::sqlite::SqliteRow;

pub struct Table<'c, T>
    where
        T: FromRow<'c, SqliteRow>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&'c SqliteRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
    where
        T: FromRow<'c, SqliteRow>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}