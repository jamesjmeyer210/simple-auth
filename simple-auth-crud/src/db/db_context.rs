use std::sync::Arc;
use sqlx::SqlitePool;
use sqlx::types::Uuid;
use crate::abs::table::Table;
use crate::entity::{ContactInfoEntity, RealmEntity, RoleEntity, UserEntity};

pub(crate) struct DbContext<'r> {
    pub realms: Arc<Table<'r, RealmEntity, String>>,
    pub roles: Arc<Table<'r, RoleEntity, String>>,
    pub users: Arc<Table<'r, UserEntity, Uuid>>,
    pub user_contacts: Arc<Table<'r, ContactInfoEntity, Vec<u8>>>,
    _pool: Arc<SqlitePool>
}

impl<'r> DbContext<'r> {
    pub async fn new(sql_url: &str) -> Result<Self, sqlx::error::Error> {
        let connection = SqlitePool::connect(sql_url).await?;
        let pool = Arc::new(connection);

        Ok(Self {
            realms: Arc::from(Table::new(pool.clone())),
            roles: Arc::new(Table::new(pool.clone())),
            users: Arc::new(Table::new(pool.clone())),
            user_contacts: Arc::new(Table::new(pool.clone())),
            _pool: pool,
        })
    }
}