use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FileRecord {
    pub id: String,
    pub filename: String,
    // pub created_at: String, // MEMO: typo bug "craeted_at"
    pub content_type: String,
    pub size: u32,
    pub link: String,
    pub error: bool,
}
