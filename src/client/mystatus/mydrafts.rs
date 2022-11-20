use crate::client::query_params::query_string;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const RESOURCE: &str = "mydrafts";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct MyDrafts {
    url: String,
    authorization_header: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GetMyDraftsResponse {
    pub app_cd: i32,
    pub offset: i32,
    pub limit: i32,
    pub total_count: i32,
    pub error: bool,
    pub records: Vec<MyDraftRecord>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MyDraftRecord {
    pub id: i32,
    pub document_id: i32,
    pub title: String,
    pub modified_date: String,
    pub create_user: Value,
    pub represent_user: Value,
    pub link: String,
}

// MEMO: represent_user の仕様に合わせる
// #[derive(Debug, Deserialize, Clone, Serialize)]
// pub struct MyDraftCreateUser {
//     id: String,
//     loginid: String,
//     name: String,
// }

// MEMO: represent_user は null ではなく {} が返却される場合がある
// #[derive(Debug, Deserialize, Clone, Serialize)]
// pub struct MyDraftRepresentUser {
//     id: String,
//     loginid: String,
//     name: String,
// }

impl MyDrafts {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        query_params: HashMap<String, String>,
    ) -> anyhow::Result<GetMyDraftsResponse> {
        let request_url = format!("{}?{}", &self.url, query_string(query_params));

        let http_client = reqwest::Client::new();
        let resp = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await?;

        match resp.json::<GetMyDraftsResponse>().await {
            Ok(mydrafts) => Ok(mydrafts),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
