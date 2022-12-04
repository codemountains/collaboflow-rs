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
