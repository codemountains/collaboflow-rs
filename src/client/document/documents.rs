use crate::query::query_string;
use anyhow::anyhow;
use std::collections::HashMap;
use crate::response::document::documents::GetDocumentsResponse;

const RESOURCE: &str = "documents";
const HEADER_KEY: &str = "X-Collaboflow-Authorization";

pub struct Documents {
    url: String,
    authorization_header: String,
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
