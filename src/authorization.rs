use data_encoding::BASE64;
use std::fmt;

pub const HEADER_KEY: &str = "X-Collaboflow-Authorization";

#[derive(Debug, Clone)]
pub enum AuthorizationType {
    ApiKey,
    Password,
}

#[derive(Debug, Clone)]
pub struct Authorization {
    authorization_type: AuthorizationType,
    user_id: String,
    value: String,
}

impl Authorization {
    pub fn with_api_key(user_id: &str, api_key: &str) -> Self {
        Self {
            authorization_type: AuthorizationType::ApiKey,
            user_id: user_id.to_string(),
            value: api_key.to_string(),
        }
    }

    pub fn with_password(user_id: &str, password: &str) -> Self {
        Self {
            authorization_type: AuthorizationType::Password,
            user_id: user_id.to_string(),
            value: password.to_string(),
        }
    }
}

impl fmt::Display for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth_header = match &self.authorization_type {
            AuthorizationType::ApiKey => format!("{}/apikey:{}", &self.user_id, &self.value),
            AuthorizationType::Password => format!("{}:{}", &self.user_id, &self.value),
        };

        let encoded: String = BASE64.encode(auth_header.as_bytes());
        write!(f, "Basic {}", encoded)
    }
}
