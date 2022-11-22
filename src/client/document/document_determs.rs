use crate::client::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const RESOURCE: &str = "documents";
const NESTED_RESOURCE: &str = "determs";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct DocumentDeterms {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentDetermsResponse {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<DetermRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DetermRecord {
    pub current: bool,
    pub comment: String,
    pub determ_date: String,
    pub determ_titlename: String,
    pub determ_usercd: String,
    pub determ_username: String,
    pub determ_status: String,
    pub document_id: i32,
    pub document_number: String,
    pub phase_number: i32,
    pub phase_title: String,
    pub represent_username: String,
    pub represent_usercd: String,
}

impl DocumentDeterms {
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
    ) -> anyhow::Result<GetDocumentDetermsResponse> {
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

        match resp.json::<GetDocumentDetermsResponse>().await {
            Ok(document) => Ok(document),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
