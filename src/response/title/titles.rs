use crate::record::title::TitleRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetTitlesResponse {
    pub status: u16,
    pub body: GetTitlesResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetTitlesResponseBody {
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<TitleRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostTitlesResponse {
    pub status: u16,
    pub body: TitleRecord,
}
