use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, Sqlite, SqlitePool, Type};
use sqlx::sqlite::SqliteRow;
use crate::abs::Entity;

pub struct Table<'r, T, K>
    where
        T: Entity<'r, K>, K : Type<Sqlite>
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&'r SqliteRow) -> Result<T, sqlx::Error>,
    _marker1: PhantomData<&'r T>,
    _marker2: PhantomData<K>
}

impl<'r, T, K> Table<'r, T, K>
    where
        T: Entity<'r, K>, K : Type<Sqlite>
{
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker1: PhantomData,
            _marker2: PhantomData
        }
    }
}