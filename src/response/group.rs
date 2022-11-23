use serde::{Deserialize, Serialize};

pub mod group;
pub mod groups;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GroupRecord {
    pub id: String,
    pub code: String,
    pub parent_code: String,
    pub name: String,
    pub description: String,
    pub order: i32,
}
