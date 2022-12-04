use crate::record::category::FormCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormsResponse {
    pub status: u16,
    pub body: GetFormsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormsResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<FormRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormRecord {
    pub id: i32,
    pub name: String,
    pub category: FormCategory,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub version: i32,
    pub create_date: String,
    pub start_date: String,
}
