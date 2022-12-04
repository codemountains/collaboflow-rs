use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GroupRecord {
    pub id: String,
    pub code: String,
    pub parent_code: String,
    pub name: String,
    pub description: String,
    pub order: i32,
}
