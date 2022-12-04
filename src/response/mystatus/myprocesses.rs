use crate::record::category::ProcessCategory;
use crate::record::form::ProcessFrom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyProcessesResponse {
    pub status: u16,
    pub body: GetMyProcessesResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyProcessesResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<MyProcessRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyProcessRecord {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub category: ProcessCategory,
    pub form: ProcessFrom,
    pub link: String,
}
