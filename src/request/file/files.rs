use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostFilesRequest {
    pub file_name: String,
    pub file_mine: String,
    pub file_data: Vec<u8>,
}

impl PostFilesRequest {
    pub fn new(file_name: String, file_mine: String, file_data: Vec<u8>) -> Self {
        Self {
            file_name,
            file_mine,
            file_data,
        }
    }
}
