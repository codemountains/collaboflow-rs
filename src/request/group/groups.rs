use serde::{Deserialize, Serialize};
use crate::record::group::NewGroupRecord;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostGroupsRequest {
    pub group: NewGroupRecord,
}

impl PostGroupsRequest {
    pub fn new(group: NewGroupRecord) -> Self {
        Self { group }
    }
}
