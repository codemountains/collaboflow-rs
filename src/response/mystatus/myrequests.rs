use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyRequestsResponse {
    pub status: u16,
    pub body: GetMyRequestsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyRequestsResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub decision_count: i32,
    pub error: bool,
    pub records: Vec<MyRequestRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyRequestRecord {
    pub id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub request_usercd: String,
    pub request_name: String,
    pub request_date: String,
    pub flow_status: String,
    pub link: String,
}
