use crate::record::user::NewUserRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostUsersRequest {
    pub user: NewUserRecord,
}

impl PostUsersRequest {
    pub fn new(user: NewUserRecord) -> Self {
        Self { user }
    }
}
