use serde::{Deserialize, Serialize};
use crate::response::document::RequestTitle;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentsResponse {
    pub status: u16,
    pub body: GetDocumentsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentsResponseBody {
    pub app_cd: i32,
    pub processes_id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub request_username: String,
    pub request_usercd: String,
    pub request_groupname: String,
    pub request_groupcd: String,
    pub request_titles: Vec<RequestTitle>,
    pub represent_username: String,
    pub represent_usercd: String,
    pub flow_status: String,
    pub request_date: String,
    pub end_date: String,
    pub link: String,
    pub error: bool,
}
