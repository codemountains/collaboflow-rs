use serde::{Deserialize, Serialize};

pub mod user_one;
pub mod users;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserRecord {
    pub id: String,
    pub userid: String,
    pub name: String,
    pub name_reading: String,
    pub employee_code: String,
    pub phone: String,
    pub phone_mobile: String,
    pub fax: String,
    pub email: String,
    pub email_mobile: String,
    pub password_change_required: String,
    pub lockout: String,
    pub order: i32,
    pub admin: String,
    pub groups: Vec<String>,
    pub titles: Vec<String>,
    pub extra1: String,
    pub extra2: String,
    pub extra3: String,
    pub extra4: String,
    pub extra5: String,
}
