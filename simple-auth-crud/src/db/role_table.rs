use crate::abs::table::Table;
use crate::entity::RoleEntity;

impl<'r> Table<'r, RoleEntity, String> {
    pub async fn add(&self, model: &RoleEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `users` (`name`, `max`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?, ?)
            "#)
            .bind(&model.name)
            .bind(&model.max)
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn all(&self) -> Result<Vec<RoleEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `name`, `max`, `created_on`, `deleted_on`
            FROM `realms`
            WHERE `deleted_on` = NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}