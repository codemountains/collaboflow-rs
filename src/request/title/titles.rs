use crate::record::title::NewTitleRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostTitlesRequest {
    pub title: NewTitleRecord,
}

impl PostTitlesRequest {
    pub fn new(title: NewTitleRecord) -> Self {
        Self { title }
    }
}
