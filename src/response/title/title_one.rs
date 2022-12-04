use crate::record::title::TitleRecord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetTitleResponse {
    pub status: u16,
    pub body: TitleRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PutTitleResponse {
    pub status: u16,
    pub body: TitleRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeleteTitleResponse {
    pub status: u16,
    pub body: Value,
}
