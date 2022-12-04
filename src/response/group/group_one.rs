use crate::record::group::GroupRecord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetGroupResponse {
    pub status: u16,
    pub body: GroupRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PutGroupResponse {
    pub status: u16,
    pub body: GroupRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeleteGroupResponse {
    pub status: u16,
    pub body: Value,
}
