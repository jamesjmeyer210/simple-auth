use simple_auth_model::chrono::Utc;
use simple_auth_model::role::RoleUpdate;
use crate::abs::table::Table;
use crate::entity::{Count, RoleEntity};

impl<'r> Table<'r, RoleEntity> {
    pub async fn add(&self, model: &RoleEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `roles` (`name`, `max`, `realm_id`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?, ?, ?)
            "#)
            .bind(&model.name)
            .bind(model.max)
            .bind(&model.realm_id)
            .bind(model.created_on)
            .bind(model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn count_by_name(&self, name: &str) -> Result<u32, sqlx::Error> {
        sqlx::query_as(r#"
            SELECT COUNT(*) FROM `roles` AS `a`
            WHERE `a`.`deleted_on` IS NULL AND `a`.`name` = ?
            "#)
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map(|x: Count|x.into())
    }

    pub async fn all(&self) -> Result<Vec<RoleEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `name`, `max`, `realm_id`, `created_on`, `deleted_on`
            FROM `roles`
            WHERE `deleted_on` IS NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<RoleEntity, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `name`, `max`, `created_on`, `realm_id`, `deleted_on`
            FROM `roles` AS `a`
            WHERE `deleted_on` IS NULL AND `a`.`name` = ?
            "#,)
            .bind(id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn update(&self, update: &RoleUpdate) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE `roles`
            SET
                `name` = ?,
                `max` = ?
            WHERE `name` = ?
            "#)
            .bind(&update.rename)
            .bind(update.max)
            .bind(&update.name)
            .execute(&*self.pool)
            .await
            .map(|e|e.rows_affected())
    }

    pub async fn soft_delete_by_id(&self, id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE `roles`
            SET `deleted_on` = ?
            WHERE `name` = ?
            "#)
            .bind(Utc::now())
            .bind(id)
            .execute(&*self.pool)
            .await
            .map(|e|e.rows_affected())
    }
}

#[cfg(test)]
mod test {
    use simple_auth_model::Role;
    use crate::db::db_context::DbContext;
    use crate::entity::{RealmEntity, RoleEntity};

    #[sqlx::test]
    async fn all_returns_entities() {
        let db = DbContext::in_memory().await.unwrap();

        let realm = RealmEntity::from("master");
        let result = db.realms.add(&realm).await;
        assert!(result.is_ok());

        let role = Role::default().with_realm(realm.name);
        let role = RoleEntity::from(&role);

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