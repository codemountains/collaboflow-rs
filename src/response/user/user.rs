use serde::{Deserialize, Serialize};

use crate::response::user::UserRecord;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetUserResponse {
    pub status: u16,
    pub body: UserRecord,
}
