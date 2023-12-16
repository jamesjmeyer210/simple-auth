use simple_auth_model::uuid::Uuid;

pub(crate) struct ContactInfoEntity {
    pub user_id: Uuid,
    pub label: String,
    pub unique_id: Vec<u8>,
    pub enc: Vec<u8>,
    pub hash: Vec<u8>,
    pub verified: bool,
}