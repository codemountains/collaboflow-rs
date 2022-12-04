use crate::record::user::UserRecord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUserResponse {
    pub status: u16,
    pub body: UserRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PutUserResponse {
    pub status: u16,
    pub body: UserRecord,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeleteUserResponse {
    pub status: u16,
    pub body: Value,
}
