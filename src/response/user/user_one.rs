use crate::record::user::ReadOnlyUserRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUserResponse {
    pub status: u16,
    pub body: ReadOnlyUserRecord,
}
