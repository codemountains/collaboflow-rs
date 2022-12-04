use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GroupRecord {
    pub id: String,
    pub code: String,
    pub parent_code: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub order: i32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NewGroupRecord {
    pub code: String,
    pub parent_code: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub order: i32,
}

impl NewGroupRecord {
    pub fn new(
        code: &str,
        parent_code: &str,
        name: &str,
        display_name: &str,
        description: &str,
        order: i32,
    ) -> Self {
        Self {
            code: code.to_string(),
            parent_code: parent_code.to_string(),
            name: name.to_string(),
            display_name: display_name.to_string(),
            description: description.to_string(),
            order,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestGroup {
    pub id: String,
    pub code: String,
    pub name: String,
}
