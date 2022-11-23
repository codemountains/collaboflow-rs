use crate::response::group::GroupRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetGroupResponse {
    pub status: u16,
    pub body: GroupRecord,
}
