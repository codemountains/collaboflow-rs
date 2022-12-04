use crate::record::document::SimulationDetermRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentsSimulationDetermsResponse {
    pub status: u16,
    pub body: PostDocumentsSimulationDetermsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostDocumentsSimulationDetermsResponseBody {
    pub app_cd: i32,
    pub total_count: i32,
    pub offset: i32,
    pub limit: i32,
    pub error: bool,
    pub records: Vec<SimulationDetermRecord>,
}
