use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub body: ErrorResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ErrorResponseBody {
    pub error: bool,
    pub messages: Vec<String>,
}
