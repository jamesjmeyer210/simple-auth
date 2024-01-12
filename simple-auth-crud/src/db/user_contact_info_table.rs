use sqlx::{QueryBuilder, Sqlite};
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

    pub(crate) async fn add_contacts(&self, contacts: &Vec<ContactInfoEntity>) -> Result<u64,sqlx::Error>
    {
        let entries = contacts.iter().map(|i|i);

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO users_contact_info (`hash`, `user_id`, `label`, `enc`, `verified`, `created_on`, `deleted_on`)"
        );

        query_builder.push_values(entries, |mut b, entity|{
           b.push_bind(&entity.hash)
               .push_bind(&entity.user_id)
               .push_bind(&entity.label)
               .push_bind(&entity.enc)
               .push_bind(&entity.verified)
               .push_bind(&entity.created_on)
               .push_bind(&entity.deleted_on);
        });

        let query = query_builder.build();
        query.execute(&*self.pool).await.map(|x|x.rows_affected())
    }
}