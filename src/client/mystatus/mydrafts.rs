use crate::authorization::HEADER_KEY;
use crate::query::query_string;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::mystatus::mydrafts::{GetMyDraftsResponse, GetMyDraftsResponseBody};
use std::collections::HashMap;

const RESOURCE: &str = "mydrafts";

#[derive(Debug, Clone)]
pub struct MyDrafts {
    url: String,
    authorization_header: String,
}

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
    ) -> Result<GetMyDraftsResponse, ErrorResponse> {
        let request_url = format!("{}?{}", &self.url, query_string(query_params));

        let http_client = reqwest::Client::new();
        let result = http_client
            .get(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    match resp.json::<GetMyDraftsResponseBody>().await {
                        Ok(body) => Ok(GetMyDraftsResponse { status, body }),
                        Err(err) => {
                            let body = ErrorResponseBody {
                                error: true,
                                messages: vec![err.to_string()],
                            };
                            let error_response = ErrorResponse { status, body };
                            Err(error_response)
                        }
                    }
                } else {
                    match resp.json::<ErrorResponseBody>().await {
                        Ok(body) => {
                            let error_response = ErrorResponse { status, body };
                            Err(error_response)
                        }
                        Err(err) => {
                            let body = ErrorResponseBody {
                                error: true,
                                messages: vec![err.to_string()],
                            };
                            let error_response = ErrorResponse { status, body };
                            Err(error_response)
                        }
                    }
                }
            }
            Err(err) => {
                let body = ErrorResponseBody {
                    error: true,
                    messages: vec![err.to_string()],
                };
                let error_response = ErrorResponse { status: 500, body };
                Err(error_response)
            }
        }
    }
}
