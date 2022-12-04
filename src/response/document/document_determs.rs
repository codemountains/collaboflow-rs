use crate::record::document::DetermRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentDetermsResponse {
    pub status: u16,
    pub body: GetDocumentDetermsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentDetermsResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<DetermRecord>,
}
