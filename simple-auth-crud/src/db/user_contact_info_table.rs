use crate::abs::table::Table;
use crate::entity::ContactInfoEntity;

// TODO: rename users_contact_info to user_contact_info
impl <'r>Table<'r, ContactInfoEntity> {
    pub async fn add(&self, model: &ContactInfoEntity) -> Result<u64,sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users_contact_info (`hash`, `user_id`, `label`, `enc`, `verified`, `created_on`, `deleted_on`)
            VALUES(?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&model.hash)
            .bind(&model.user_id)
            .bind(&model.label)
            .bind(&model.enc)
            .bind(&model.verified)
            .bind(&model.created_on)
            .bind(&model.deleted_on)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }
}