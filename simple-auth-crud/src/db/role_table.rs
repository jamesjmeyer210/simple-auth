use crate::abs::table::Table;
use crate::entity::RoleEntity;

impl<'r> Table<'r, RoleEntity, String> {
    pub async fn add(&self, model: &RoleEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `roles` (`name`, `max`, `created_on`, `deleted_on`)
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
            FROM `roles`
            WHERE `deleted_on` IS NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::db::db_context::DbContext;
    use crate::entity::RoleEntity;

    #[sqlx::test]
    async fn all_returns_entities() {
        let db = DbContext::in_memory().await.unwrap();
        let role = RoleEntity::from("root");

        let x = db.roles.add(&role).await;
        assert!(x.is_ok());
        let x = x.unwrap();
        assert_eq!(1, x);

        let entities = db.roles.all().await;
        assert!(entities.is_ok());
        let entities = entities.unwrap();
        assert_eq!(1, entities.len());
        assert_eq!("root", entities[0].name)
    }
}