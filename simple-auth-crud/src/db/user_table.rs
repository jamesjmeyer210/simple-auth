use sqlx::types::Uuid;
use crate::abs::table::Table;
use crate::entity::UserEntity;

impl<'r> Table<'r, UserEntity, Uuid> {
    pub async fn add(&self, model: &UserEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `password`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?, ?, ?)"#)
            .bind(&model.id)
            .bind(&model.name)
            .bind(&model.password.as_ref().map(|x|x.as_bytes()))
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn all(&self) -> Result<Vec<UserEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `password`, `created_on`, `deleted_on`
            FROM `users` as `a`
            WHERE `a`.`deleted_on` IS NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbContext;
    use crate::entity::UserEntity;

    #[sqlx::test]
    async fn all_returns_entries() {
        let db = DbContext::in_memory().await.unwrap();
        let user = UserEntity::default();

        let x = db.users.add(&user).await;
        assert!(x.is_ok());

        let entities = db.users.all().await;
        assert!(x.is_ok());
        let entities = entities.unwrap();
        assert_eq!(1, entities.len());
        assert_eq!(user.name, entities[0].name);
    }
}