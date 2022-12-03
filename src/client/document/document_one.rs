use crate::authorization::HEADER_KEY;
use crate::request::document::document_one::PutDocumentStatusRequest;
use crate::response::document::document_one::{DeleteDocumentResponse, GetDocumentResponse, GetDocumentResponseBody, PutDocumentStatusResponse};
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::Query;
use serde::Serialize;

const RESOURCE: &str = "documents";

#[derive(Debug, Clone)]
pub struct Document {
    url: String,
    authorization_header: String,
}

impl Document {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        document_id: i32,
        query: Query,
    ) -> Result<GetDocumentResponse, ErrorResponse> {
        let request_url = format!("{}/{}?{}", &self.url, document_id, query);

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
                    match resp.json::<GetDocumentResponseBody>().await {
                        Ok(body) => Ok(GetDocumentResponse { status, body }),
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

    pub async fn put<T: Serialize>(
        &self,
        document_id: i32,
        query: Query,
        request: PutDocumentStatusRequest<T>,
    ) -> Result<PutDocumentStatusResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, document_id);

        let http_client = reqwest::Client::new();
        let result = http_client
            .put(request_url)
            .query(&query.to_queries())
            .json(&request)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    Ok(PutDocumentStatusResponse { status })
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

    pub async fn delete(
        &self,
        document_id: i32,
        query: Query,
    ) -> Result<DeleteDocumentResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, document_id);

        let http_client = reqwest::Client::new();
        let result = http_client
            .delete(request_url)
            .query(&query.to_queries())
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    Ok(DeleteDocumentResponse { status })
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
