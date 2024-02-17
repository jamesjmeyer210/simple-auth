use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoleUpdate {
    pub name: String,
    pub rename: String,
    pub max: Option<u32>
}