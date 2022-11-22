use crate::client::query::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const RESOURCE: &str = "forms";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct Forms {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormsResponse {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<FormRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormRecord {
    pub id: i32,
    pub name: String,
    pub category: FormCategory,
    pub description: String,
    pub status: String,
    pub r#type: String,
    pub version: i32,
    pub create_date: String,
    pub start_date: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FormCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetFormsErrorResponse {
    pub error: bool,
    pub messages: Vec<String>,
}

impl Forms {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        query_params: HashMap<String, String>,
    ) -> anyhow::Result<GetFormsResponse> {
        let request_url = format!("{}?{}", &self.url, query_string(query_params));

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetFormsResponse>().await {
            Ok(forms) => Ok(forms),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
