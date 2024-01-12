use crate::abs::table::Table;
use crate::entity::{Count, RealmEntity};

impl<'r> Table<'r, RealmEntity> {
    pub async fn add(&self, model: &RealmEntity) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `realms` (`name`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?)
            "#)
            .bind(&model.name)
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn count(&self) -> Result<u32, sqlx::Error> {
        sqlx::query_as("SELECT COUNT(*) FROM `realms` as `a` WHERE `a`.`deleted_on` IS NULL",)
            .fetch_one(&*self.pool)
            .await
            .map(|x: Count|x.into())
    }

    pub async fn count_by_name(&self, name: &str) -> Result<u32, sqlx::Error> {
        sqlx::query_as(r#"
            SELECT COUNT(*) FROM `realms` as `a`
            WHERE `a`.`deleted_on` IS NULL AND `a`.`name` = ?
            "#)
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map(|x: Count|x.into())
    }

    pub async fn all(&self) -> Result<Vec<RealmEntity>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `name`, `created_on`, `deleted_on`
            FROM `realms` as `a`
            WHERE `a`.`deleted_on` IS NULL
            "#,)
            .fetch_all(&*self.pool)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbContext;
    use crate::entity::RealmEntity;

    #[sqlx::test]
    async fn add_returns_ok() {
        let db = DbContext::in_memory().await;
        assert!(db.is_ok());
        let db = db.unwrap();

        let realm = RealmEntity::from("master");
        let x = db.realms.add(&realm).await;

        assert!(x.is_ok());
        let x = x.unwrap();
        assert_eq!(1, x);

        let c = db.realms.count().await;
        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c);
    }

    #[sqlx::test]
    async fn all_returns_entries() {
        let db = DbContext::in_memory().await.unwrap();
        let realm = RealmEntity::from("master");

        let x = db.realms.add(&realm).await.unwrap();
        assert_eq!(1, x);

        let entities = db.realms.all().await;
        assert!(entities.is_ok());

        let entities = entities.unwrap();
        assert_eq!(1, entities.len());

        assert_eq!("master", entities[0].name);
    }
}