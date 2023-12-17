use crate::abs::table::Table;
use crate::entity::RealmEntity;

impl<'r> Table<'r, RealmEntity, String> {
    pub async fn add(&self, model: &RealmEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `users` (`name`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?)
            "#)
            .bind(&model.name)
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn all(&self) -> Result<Vec<RealmEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `name`, `created_on`, `deleted_on`
            FROM `realms`
            WHERE `deleted_on` = NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}