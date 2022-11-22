use crate::client::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const RESOURCE: &str = "myprocesses";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct MyProcesses {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyProcessesResponse {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Value,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyProcessRecord {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub category: MyProcessCategory,
    pub form: MyProcessFrom,
    pub link: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyProcessCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyProcessFrom {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}

impl MyProcesses {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        query_params: HashMap<String, String>,
    ) -> anyhow::Result<GetMyProcessesResponse> {
        let request_url = format!("{}?{}", &self.url, query_string(query_params));

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetMyProcessesResponse>().await {
            Ok(myprocesses) => Ok(myprocesses),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
