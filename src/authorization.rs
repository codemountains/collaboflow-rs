use data_encoding::BASE64;
use std::fmt;

pub enum AuthorizationType {
    Password,
    ApiKey,
}

pub struct CollaboflowAuthorization {
    authorization_type: AuthorizationType,
    user_id: String,
    value: String,
}

impl CollaboflowAuthorization {
    pub fn new(authorization_type: AuthorizationType, user_id: &str, value: &str) -> Self {
        Self {
            authorization_type,
            user_id: user_id.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for CollaboflowAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth_header = match &self.authorization_type {
            AuthorizationType::Password => format!("{}:{}", &self.user_id, &self.value),
            AuthorizationType::ApiKey => format!("{}/apikey:{}", &self.user_id, &self.value),
        };

        let encoded: String = BASE64.encode(auth_header.as_bytes());
        write!(f, "Basic {}", encoded)
    }
}
