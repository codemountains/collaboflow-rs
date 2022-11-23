use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormPartsResponse {
    pub status: u16,
    pub body: GetFormPartsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormPartsResponseBody {
    pub app_cd: i32,
    pub version: i32,
    pub total_count: i32,
    pub error: bool,
    pub parts: Value,
}
