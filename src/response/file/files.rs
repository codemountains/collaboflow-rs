use crate::record::file::FileRecord;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostFilesResponse {
    pub status: u16,
    pub body: FileRecord,
}

#[derive(Debug, Clone)]
pub struct GetFileDownloadResponse {
    pub status: u16,
    pub body: Bytes,
}
