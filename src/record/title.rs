use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TitleRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub level: i32,
}
