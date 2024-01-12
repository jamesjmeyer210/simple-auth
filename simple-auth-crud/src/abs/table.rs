use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};
use sqlx::sqlite::SqliteRow;

pub struct Table<'r, T>
    where
        T: FromRow<'r, SqliteRow>
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&'r SqliteRow) -> Result<T, sqlx::Error>,
    _marker1: PhantomData<&'r T>,
}

impl<'r, T> Table<'r, T>
    where
        T: FromRow<'r, SqliteRow>
{
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker1: PhantomData,
        }
    }
}