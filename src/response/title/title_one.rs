use crate::record::title::TitleRecord;
use serde::{Deserialize, Serialize};

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
