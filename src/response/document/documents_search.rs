use crate::record::document::SearchedDocumentRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentsSearchResponse {
    pub status: u16,
    pub body: PostDocumentsSearchResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentsSearchResponseBody {
    pub app_cd: i32,
    pub total_count: i32,
    pub offset: i32,
    pub limit: i32,
    pub error: bool,
    pub records: Vec<SearchedDocumentRecord>,
}
