use sqlx::types::Uuid;
use crate::abs::table::Table;
use crate::entity::UserEntity;

impl<'r> Table<'r, UserEntity, Uuid> {
    pub async fn add(&self, model: &UserEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `password`, `public_key`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?, ?, ?, ?)"#)
            .bind(&model.id)
            .bind(&model.name)
            .bind(&model.password.as_ref().map(|x|x.as_bytes()))
            .bind(&model.public_key)
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn all(&self) -> Result<Vec<UserEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `password`, `public_key`, `created_on`, `deleted_on`
            FROM `users`
            WHERE `deleted_on` = NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}