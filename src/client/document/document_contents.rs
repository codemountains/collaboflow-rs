use crate::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const RESOURCE: &str = "documents";
const NESTED_RESOURCE: &str = "contents";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct DocumentContents {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentContentsResponse {
    pub app_cd: i32,
    pub processes_id: i32,
    pub error: bool,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub flow_status: String,
    pub link: String,
    pub request_date: String,
    pub request_group: DocumentContentRequestGroup,
    pub request_titles: Vec<DocumentContentRequestTitle>,
    pub request_user: DocumentContentRequestUser,
    pub represent_user: DocumentContentRepresentUser,
    pub end_date: String,
    pub contents: Value,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DocumentContentRequestGroup {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DocumentContentRequestTitle {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DocumentContentRequestUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DocumentContentRepresentUser {
    pub id: String,
    pub loginid: String,
    pub name: String,
}

impl DocumentContents {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        document_id: i32,
        query_params: HashMap<String, String>,
    ) -> anyhow::Result<GetDocumentContentsResponse> {
        let request_url = format!(
            "{}/{}/{}?{}",
            &self.url,
            document_id,
            NESTED_RESOURCE,
            query_string(query_params)
        );

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetDocumentContentsResponse>().await {
            Ok(document) => Ok(document),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
