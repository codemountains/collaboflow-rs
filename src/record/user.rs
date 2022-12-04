use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUserRecord {
    pub userid: String,
    pub name: String,
    pub name_reading: String,
    pub employee_code: String,
    pub phone: String,
    pub phone_mobile: String,
    pub fax: String,
    pub email: String,
    pub email_mobile: String,
    pub password_change_required: bool,
    pub lockout: bool,
    pub order: i32,
    pub admin: bool,
    pub groups: Vec<String>,
    pub titles: Vec<String>,
    pub extra1: String,
    pub extra2: String,
    pub extra3: String,
    pub extra4: String,
    pub extra5: String,
    pub password: String,
}

impl NewUserRecord {
    pub fn new(
        userid: &str,
        name: &str,
        name_reading: &str,
        employee_code: &str,
        phone: &str,
        phone_mobile: &str,
        fax: &str,
        email: &str,
        email_mobile: &str,
        password_change_required: bool,
        lockout: bool,
        order: i32,
        admin: bool,
        groups: Vec<String>,
        titles: Vec<String>,
        extra1: &str,
        extra2: &str,
        extra3: &str,
        extra4: &str,
        extra5: &str,
        password: &str,
    ) -> Self {
        Self {
            userid: userid.to_string(),
            name: name.to_string(),
            name_reading: name_reading.to_string(),
            employee_code: employee_code.to_string(),
            phone: phone.to_string(),
            phone_mobile: phone_mobile.to_string(),
            fax: fax.to_string(),
            email: email.to_string(),
            email_mobile: email_mobile.to_string(),
            password_change_required,
            lockout,
            order,
            admin,
            groups,
            titles,
            extra1: extra1.to_string(),
            extra2: extra2.to_string(),
            extra3: extra3.to_string(),
            extra4: extra4.to_string(),
            extra5: extra5.to_string(),
            password: password.to_string(),
        }
    }
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
