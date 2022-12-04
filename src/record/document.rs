use crate::response::{RepresentUser, RequestGroup, RequestUser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SimulationDetermRecord {
    pub current: bool,
    pub comment: String,
    pub determ_date: String,
    pub determ_titlename: String,
    pub determ_usercd: String,
    pub determ_username: String,
    pub determ_status: String,
    pub phase_number: i32,
    pub phase_title: String,
    pub represent_username: String,
    pub represent_usercd: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SearchedDocumentRecord {
    pub form_id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub flow_status: String,
    pub request_date: String,
    pub end_date: String,
    pub link: String,
    pub request_user: RequestUser,
    pub request_group: RequestGroup,
    pub represent_user: RepresentUser,
}
