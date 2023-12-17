use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};
use sqlx::sqlite::SqliteRow;

pub struct JoinTable<'c, T1, T2>
    where
        T1: FromRow<'c, SqliteRow>,
        T2: FromRow<'c, SqliteRow>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: (
        fn(&'c SqliteRow) -> Result<T1, sqlx::Error>,
        fn(&'c SqliteRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
    where
        T1: FromRow<'c, SqliteRow>,
        T2: FromRow<'c, SqliteRow>,
{
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}