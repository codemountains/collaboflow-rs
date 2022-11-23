use serde::{Deserialize, Serialize};

pub mod document_contents;
pub mod document_determs;
pub mod documents;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestTitle {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestGroup {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RepresentUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}
