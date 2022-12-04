use crate::record::user::UserRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUsersResponse {
    pub status: u16,
    pub body: GetUsersResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUsersResponseBody {
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<UserRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostUsersResponse {
    pub status: u16,
    pub body: UserRecord,
}
