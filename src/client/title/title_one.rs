use crate::authorization::HEADER_KEY;
use crate::client::title::RESOURCE_V1_TITLES;
use crate::record::title::TitleRecord;
use crate::request::title::title_one::PutTitleRequest;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::title::title_one::{DeleteTitleResponse, GetTitleResponse, PutTitleResponse};
use crate::Query;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Title {
    url: String,
    authorization_header: String,
}

impl Title {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_TITLES,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        title_id: &str,
        query: Query,
    ) -> Result<GetTitleResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, title_id);

        let http_client = reqwest::Client::new();
        let result = http_client
            .get(request_url)
            .query(&query.to_queries())
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    match resp.json::<TitleRecord>().await {
                        Ok(body) => Ok(GetTitleResponse { status, body }),
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
        title_id: &str,
        query: Query,
        request: PutTitleRequest<T>,
    ) -> Result<PutTitleResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, title_id);

        let http_client = reqwest::Client::new();
        let result = http_client
            .put(request_url)
            .query(&query.to_queries())
            .json(&request.title)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    match resp.json::<TitleRecord>().await {
                        Ok(body) => Ok(PutTitleResponse { status, body }),
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

    pub async fn delete(
        &self,
        title_id: &str,
        query: Query,
    ) -> Result<DeleteTitleResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, title_id);

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
                    match resp.json::<Value>().await {
                        Ok(body) => Ok(DeleteTitleResponse { status, body }),
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
