use crate::record::user::UserRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostUsersRequest {
    pub user: UserRecord,
}

impl PostUsersRequest {
    pub fn new(user: UserRecord) -> Self {
        Self { user }
    }
}
