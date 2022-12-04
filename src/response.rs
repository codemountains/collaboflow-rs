use serde::{Deserialize, Serialize};

pub mod document;
pub mod error;
pub mod form;
pub mod group;
pub mod mystatus;
pub mod title;
pub mod user;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProcessCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProcessFrom {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormCategory {
    pub id: i32,
    pub name: String,
}
