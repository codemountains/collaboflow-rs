use crate::record::title::RequestTitle;
use crate::response::{RepresentUser, RequestUser};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::record::group::RequestGroup;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentContentsResponse {
    pub status: u16,
    pub body: GetDocumentContentsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentContentsResponseBody {
    pub app_cd: i32,
    pub processes_id: i32,
    pub error: bool,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub flow_status: String,
    pub link: String,
    pub request_date: String,
    pub request_group: RequestGroup,
    pub request_titles: Vec<RequestTitle>,
    pub request_user: RequestUser,
    pub represent_user: RepresentUser,
    pub end_date: String,
    pub contents: Value,
}
