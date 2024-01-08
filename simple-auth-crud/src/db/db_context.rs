use std::any::{Any, TypeId};
use std::sync::Arc;
use sqlx::migrate::{MigrateError};
use sqlx::SqlitePool;
use sqlx::types::Uuid;
use crate::abs::join_table::JoinTable;
use crate::abs::table::Table;
use crate::entity::{ContactInfoEntity, RealmEntity, RoleEntity, UserEntity};

pub struct DbContext<'r> {
    pub(crate) realms: Arc<Table<'r, RealmEntity>>,
    pub(crate) roles: Arc<Table<'r, RoleEntity>>,
    pub(crate) roles_to_realms: Arc<JoinTable<'r, RoleEntity, RealmEntity>>,
    pub(crate) users: Arc<Table<'r, UserEntity>>,
    pub(crate) user_contacts: Arc<Table<'r, ContactInfoEntity>>,
    _pool: Arc<SqlitePool>
}

impl<'r> DbContext<'r> {
    pub async fn new(sql_url: &str) -> Result<Self, sqlx::error::Error> {
        let connection = SqlitePool::connect(sql_url).await?;
        let pool = Arc::new(connection);

        Ok(Self {
            realms: Arc::from(Table::new(pool.clone())),
            roles: Arc::new(Table::new(pool.clone())),
            roles_to_realms: Arc::new(JoinTable::new(pool.clone())),
            users: Arc::new(Table::new(pool.clone())),
            user_contacts: Arc::new(Table::new(pool.clone())),
            _pool: pool,
        })
    }

    pub async fn migrate(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("../migrations")
            .run(&*self._pool)
            .await
    }

    pub async fn in_memory() -> Result<Self, sqlx::Error> {
        let db = Self::new("sqlite::memory:").await?;
        db.migrate().await.expect("migration failed");
        Ok(db)
    }

    pub fn get_crud<T: for<'a> From<&'a DbContext<'r>>>(&self) -> T {
        T::from(self)
    }
}

#[cfg(test)]
mod test {
    use crate::crud::RealmCrud;
    use super::DbContext;

    #[sqlx::test]
    async fn new_returns_ok(){
        let db = DbContext::new("sqlite::memory:").await;
        assert!(db.is_ok())
    }

    #[sqlx::test]
    async fn migrate_returns_ok(){
        let db = DbContext::new("sqlite::memory:").await;
        assert!(db.is_ok());

        let db = db.unwrap();
        let x = db.migrate().await;
        assert!(x.is_ok())
    }

    #[sqlx::test]
    async fn get_crud_returns_realm_crud(){
        let db = DbContext::new("sqlite::memory:").await;
        assert!(db.is_ok());
        let db = db.unwrap();

        let crud = db.get_crud::<RealmCrud>();
    }
}