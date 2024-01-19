use sqlx::{query_as, QueryBuilder, Sqlite};
use sqlx::types::Uuid;
use crate::abs::join_table::JoinTable;
use crate::entity::{RealmEntity, UserEntity};

impl <'r>JoinTable<'r, UserEntity, RealmEntity> {
    pub(crate) async fn add_realms_to_user(&self, user_id: &Uuid, realms: &Vec<&String>)
        -> Result<u64,sqlx::Error>
    {
        let relations = realms.iter().map(|realm|(user_id, realm));

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO `users_to_realms` (`user_id`, `realm_id`)"
        );

        query_builder.push_values(relations.take(realms.len()), |mut b, relation|{
            b.push_bind(relation.0)
                .push_bind(relation.1);
        });

        let query = query_builder.build();
        query.execute(&*self.pool).await.map(|x|x.rows_affected())
    }

    pub(crate) async fn get_realms_by_user_id(&self, user_id: &Uuid) -> Result<Vec<RealmEntity>,sqlx::Error> {
        query_as(
            r#"
            select `name`, `created_on`, `deleted_on`
            from `realms` as `a`
            where `a`.`name` = (
                select `realm_id`
                from `users_to_realms` as `b`
                where `b`.`user_id` = ?
            )
            "#)
            .bind(user_id)
            .fetch_all(&*self.pool)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbContext;

    #[sqlx::test]
    async  fn get_realms_by_user_id_returns_realms() {
        let db = DbContext::in_memory().await.unwrap();
        let user = db.init_default_unchecked().await;

        let realms = db.users_to_realms.get_realms_by_user_id(&user.id).await;
        assert!(realms.is_ok());

        let realms = realms.unwrap();
        assert_eq!(1, realms.len());
    }
}