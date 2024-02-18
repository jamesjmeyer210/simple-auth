use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;

pub struct Count(u32);

impl From<Count> for u32 {
    fn from(val: Count) -> Self {
        val.0
    }
}

impl <'r>FromRow<'r, SqliteRow> for Count {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Count(row.try_get(0)?))
    }
}