use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutDocumentStatusRequest<T>
where
    T: Serialize,
{
    pub action: String,
    pub comment: Option<String>,
    pub phase_number: Option<String>,
    pub request_groupcode: Option<String>,
    pub title: Option<String>,
    pub document: Option<T>,
}

impl<T: Serialize> PutDocumentStatusRequest<T> {
    pub fn new(
        action: &str,
        comment: Option<&str>,
        phase_number: Option<&str>,
        request_groupcode: Option<&str>,
        title: Option<&str>,
        document: Option<T>,
    ) -> Self {
        let document = if let Some(d) = document {
            Some(d)
        } else {
            None
        };

        Self {
            action: action.to_string(),
            comment: comment.map(str::to_string),
            phase_number: phase_number.map(str::to_string),
            request_groupcode: request_groupcode.map(str::to_string),
            title: title.map(str::to_string),
            document,
        }
    }
}
