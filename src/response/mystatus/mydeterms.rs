use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyDetermsResponse {
    pub status: u16,
    pub body: GetMyDetermsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyDetermsResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<MyDetermRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyDetermRecord {
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
