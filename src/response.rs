use serde::{Deserialize, Serialize};

pub mod document;
pub mod error;
pub mod form;
pub mod group;
pub mod mystatus;
pub mod title;
pub mod user;

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
pub struct CreateUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct RepresentUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct EmptyRepresentUser {}
