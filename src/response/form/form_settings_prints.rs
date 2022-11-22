use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormSettingsPrintsResponse {
    pub status: u16,
    pub body: GetFormSettingsPrintsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormSettingsPrintsResponseBody {
    pub app_cd: i32,
    pub error: bool,
    pub prints: Vec<FormSettingsPrintsRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormSettingsPrintsRecord {
    pub id: i32,
    pub name: String,
    pub form_version: i32,
    pub output_item_type: String,
    pub extentions: Vec<String>,
    pub file_name: String,
}
