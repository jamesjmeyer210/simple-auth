use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;

pub struct Count(u32);

impl Into<u32> for Count {
    fn into(self) -> u32 {
        self.0
    }
}

impl <'r>FromRow<'r, SqliteRow> for Count {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Count(row.try_get(0)?))
    }
}