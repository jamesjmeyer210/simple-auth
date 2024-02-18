use crate::abs::table::Table;
use crate::entity::{Count, SecretEntity};

impl <'r>Table<'r, SecretEntity> {
    pub async fn add(&self, model: SecretEntity) -> Result<u64,sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `secrets` (`key`, `value_enc`, `expires_on`)
            VALUES(?, ?, ?)
            "#)
            .bind(&model.key)
            .bind(&model.value_enc)
            .bind(model.expires_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn contains(&self, key: &str) -> Result<bool,sqlx::Error> {
        sqlx::query_as("SELECT COUNT(*) FROM `secrets` as `a` WHERE `a`.`key` = ?")
            .bind(key)
            .fetch_one(&*self.pool)
            .await
            .map(|x: Count|<Count as Into<u32>>::into(x) == 1)
    }

    pub async fn get(&self, key: &str) -> Result<SecretEntity,sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `key`, `value_enc`, `expires_on`
            FROM `secrets` as `a`
            WHERE `a`.`key` = ?
            "#)
            .bind(key)
            .fetch_one(&*self.pool)
            .await
    }
}