use crate::record::user::ReadOnlyUserRecord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUserResponse {
    pub status: u16,
    pub body: ReadOnlyUserRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeleteUserResponse {
    pub status: u16,
    pub body: Value,
}
