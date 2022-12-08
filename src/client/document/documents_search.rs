use crate::authorization::HEADER_KEY;
use crate::client::document::RESOURCE_V1_DOCUMENTS;
use crate::request::document::documents_search::PostDocumentsSearchRequest;
use crate::response::document::documents_search::{
    PostDocumentsSearchResponse, PostDocumentsSearchResponseBody,
};
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use serde::Serialize;

const ACTION: &str = "/search";

#[derive(Debug, Clone)]
pub struct DocumentsSearch {
    url: String,
    authorization_header: String,
}

impl DocumentsSearch {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_DOCUMENTS + ACTION,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn post<T: Serialize>(
        &self,
        request: PostDocumentsSearchRequest<T>,
    ) -> Result<PostDocumentsSearchResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .header(HEADER_KEY, &self.authorization_header)
            .json(&request.data)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    match resp.json::<PostDocumentsSearchResponseBody>().await {
                        Ok(body) => Ok(PostDocumentsSearchResponse { status, body }),
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
