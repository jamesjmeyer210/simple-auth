mod db_context;
mod realm_table;
mod user_table;
mod role_table;
mod roles_to_realms_table;

pub(crate) type DbContext<'r> = db_context::DbContext<'r>;