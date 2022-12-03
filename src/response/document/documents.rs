use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentResponse {
    pub status: u16,
    pub body: PostDocumentResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentResponseBody {
    pub error: bool,
    pub app_cd: i32,
    pub processes_id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub request_username: String,
    pub request_usercd: String,
    pub request_groupname: String,
    pub request_groupcd: String,
    pub represent_username: String,
    pub represent_usercd: String,
    pub request_date: String,
    pub link: String,
}
