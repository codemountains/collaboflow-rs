use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TitleRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub level: i32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NewTitleRecord {
    pub code: String,
    pub name: String,
    pub description: String,
    pub level: i32,
}

impl NewTitleRecord {
    pub fn new(code: &str, name: &str, description: &str, level: i32) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            level,
        }
    }
}
