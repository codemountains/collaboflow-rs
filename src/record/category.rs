use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProcessCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormCategory {
    pub id: i32,
    pub name: String,
}
