use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDocumentRequest<T>
where
    T: Serialize,
{
    pub processes_id: i32,
    pub request_userid: Option<String>,
    pub request_groupcode: Option<String>,
    pub action: Option<String>,
    pub app_cd: i32,
    pub title: Option<String>,
    pub document: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PostDocumentRequest<T> {
    pub fn new(
        processes_id: i32,
        request_userid: Option<&str>,
        request_groupcode: Option<&str>,
        action: Option<&str>,
        app_cd: i32,
        title: Option<&str>,
        document: T,
    ) -> Self {
        Self {
            processes_id,
            request_userid: request_userid.map(str::to_string),
            request_groupcode: request_groupcode.map(str::to_string),
            action: action.map(str::to_string),
            app_cd,
            title: title.map(str::to_string),
            document,
            _phantom: Default::default(),
        }
    }
}
