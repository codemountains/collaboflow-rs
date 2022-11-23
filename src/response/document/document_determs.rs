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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DetermRecord {
    pub current: bool,
    pub comment: String,
    pub determ_date: String,
    pub determ_titlename: String,
    pub determ_usercd: String,
    pub determ_username: String,
    pub determ_status: String,
    pub document_id: i32,
    pub document_number: String,
    pub phase_number: i32,
    pub phase_title: String,
    pub represent_username: String,
    pub represent_usercd: String,
}
