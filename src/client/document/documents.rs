use crate::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const RESOURCE: &str = "documents";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct Documents {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetDocumentsResponse {
    pub app_cd: i32,
    pub processes_id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub request_username: String,
    pub request_usercd: String,
    pub request_groupname: String,
    pub request_groupcd: String,
    pub request_titles: Vec<RequestTitles>,
    pub represent_username: String,
    pub represent_usercd: String,
    pub flow_status: String,
    pub request_date: String,
    pub end_date: String,
    pub link: String,
    pub error: bool,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestTitles {
    id: String,
    code: String,
    name: String,
}

impl Documents {
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
    ) -> anyhow::Result<GetDocumentsResponse> {
        let request_url = format!(
            "{}/{}?{}",
            &self.url,
            document_id,
            query_string(query_params)
        );

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetDocumentsResponse>().await {
            Ok(document) => Ok(document),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
