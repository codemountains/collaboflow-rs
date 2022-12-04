use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProcessFrom {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}
