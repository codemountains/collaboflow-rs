use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

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

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = "".to_string();
        for err in &self.body.messages {
            if msg.is_empty() {
                msg += err;
            } else {
                msg += format!("\n{}", msg).as_str();
            }
        }
        write!(f, "{}", msg)
    }
}

impl Error for ErrorResponse {}
