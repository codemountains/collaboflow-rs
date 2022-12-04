use crate::record::group::GroupRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetGroupsResponse {
    pub status: u16,
    pub body: GetGroupsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetGroupsResponseBody {
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<GroupRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostGroupsResponse {
    pub status: u16,
    pub body: GroupRecord,
}
