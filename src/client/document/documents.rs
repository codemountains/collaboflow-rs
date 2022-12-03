use crate::authorization::HEADER_KEY;
use crate::request::document::documents::PostDocumentsRequest;
use crate::response::document::documents::{PostDocumentsResponse, PostDocumentsResponseBody};
use crate::response::error::{ErrorResponse, ErrorResponseBody};
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

    pub async fn post<T: Serialize>(
        &self,
        request: PostDocumentsRequest<T>,
    ) -> Result<PostDocumentsResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .json(&request)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 201 {
                    match resp.json::<PostDocumentsResponseBody>().await {
                        Ok(body) => Ok(PostDocumentsResponse { status, body }),
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
