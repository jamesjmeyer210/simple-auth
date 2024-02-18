use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct AddRoleDto {
    pub name: String,
    pub max: Option<u32>,
    pub realm: String,
}