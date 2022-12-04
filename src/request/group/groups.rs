use crate::record::group::NewGroupRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostGroupsRequest {
    pub group: NewGroupRecord,
}

impl PostGroupsRequest {
    pub fn new(group: NewGroupRecord) -> Self {
        Self { group }
    }
}
