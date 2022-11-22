use crate::client::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const RESOURCE: &str = "myrequests";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct MyRequests {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyRequestsResponse {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub decision_count: i32,
    pub error: bool,
    pub records: Vec<MyRequestRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyRequestRecord {
    pub id: i32,
    pub document_id: i32,
    pub document_number: String,
    pub title: String,
    pub request_usercd: String,
    pub request_name: String,
    pub request_date: String,
    pub flow_status: String,
    pub link: String,
}

impl MyRequests {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        query_params: HashMap<String, String>,
    ) -> anyhow::Result<GetMyRequestsResponse> {
        let request_url = format!("{}?{}", &self.url, query_string(query_params));

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetMyRequestsResponse>().await {
            Ok(myrequests) => Ok(myrequests),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
