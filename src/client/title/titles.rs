use crate::authorization::HEADER_KEY;
use crate::client::title::RESOURCE_V1_TITLES;
use crate::record::title::TitleRecord;
use crate::request::title::titles::PostTitlesRequest;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::title::titles::{
    GetTitlesResponse, GetTitlesResponseBody, GetTitlesResponseBodyWithFields,
    GetTitlesResponseWithFields, PostTitlesResponse,
};
use crate::Query;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Titles {
    url: String,
    authorization_header: String,
}

impl Titles {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_TITLES,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(&self, query: Query) -> Result<GetTitlesResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

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
                    match resp.json::<GetTitlesResponseBody>().await {
                        Ok(body) => Ok(GetTitlesResponse { status, body }),
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

    pub async fn get_with_fields<T: for<'a> Deserialize<'a>>(
        &self,
        query: Query,
    ) -> Result<GetTitlesResponseWithFields<T>, ErrorResponse> {
        let request_url = format!("{}", &self.url);

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
                    match resp.json::<GetTitlesResponseBodyWithFields<T>>().await {
                        Ok(body) => Ok(GetTitlesResponseWithFields { status, body }),
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

    pub async fn post(
        &self,
        request: PostTitlesRequest,
    ) -> Result<PostTitlesResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .json(&request.title)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 201 {
                    match resp.json::<TitleRecord>().await {
                        Ok(body) => Ok(PostTitlesResponse { status, body }),
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
