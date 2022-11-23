use crate::response::{CreateUser, EmptyRepresentUser, RepresentUser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyDraftsResponse {
    pub status: u16,
    pub body: GetMyDraftsResponseBody,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyDraftsResponseBody {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<MyDraftRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyDraftRecord {
    pub id: i32,
    pub document_id: i32,
    pub title: String,
    pub modified_date: String,
    pub create_user: CreateUser,
    pub represent_user: MyDraftRepresentUser,
    pub link: String,
}

// MEMO: represent_user は null ではなく {} が返却される場合があるため、enum で共用体を定義

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MyDraftRepresentUser {
    RepresentUser(RepresentUser),
    EmptyRepresentUser(EmptyRepresentUser),
}
