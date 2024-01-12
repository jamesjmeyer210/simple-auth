use sqlx::{QueryBuilder, Sqlite};
use crate::abs::join_table::JoinTable;
use crate::entity::{RealmEntity, RoleEntity};

impl <'r>JoinTable<'r, RoleEntity, RealmEntity> {

    pub(crate) async fn add_realms_to_role(&self, role: &String, realms: &Vec<&String>)
        -> Result<u64,sqlx::Error>
    {
        let relations = realms.iter().map(|realm|(role, realm));

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO `roles_to_realms` (`role_name`, `realm_name`)"
        );

        query_builder.push_values(relations.take(realms.len()), |mut b, relation|{
           b.push_bind(relation.0)
               .push_bind(relation.1);
        });

        let query = query_builder.build();
        query.execute(&*self.pool).await.map(|x|x.rows_affected())
    }

}