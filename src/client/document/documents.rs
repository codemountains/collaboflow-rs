use crate::authorization::HEADER_KEY;
use crate::request::document::documents::PostDocumentRequest;
use serde::Serialize;

const RESOURCE: &str = "documents";

#[derive(Debug, Clone)]
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

    pub async fn post<T: Serialize>(&self, request: PostDocumentRequest<T>) -> Result<(), ()> {
        let request_url = format!("{}", &self.url);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .json(&request)
            .send()
            .await;

        match result {
            Ok(resp) => println!("{:?}", resp.text().await),
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    }
}
