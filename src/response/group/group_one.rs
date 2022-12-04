use crate::record::group::GroupRecord;
use serde::{Deserialize, Serialize};

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
