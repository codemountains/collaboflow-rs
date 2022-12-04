use crate::authorization::HEADER_KEY;
use crate::record::user::UserRecord;
use crate::request::user::user_one::PutUserRequest;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::user::user_one::{DeleteUserResponse, GetUserResponse};
use crate::Query;
use serde::Serialize;
use serde_json::Value;

const RESOURCE: &str = "users";

#[derive(Debug, Clone)]
pub struct User {
    url: String,
    authorization_header: String,
}

impl User {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(&self, user_id: &str, query: Query) -> Result<GetUserResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, user_id);

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
                    match resp.json::<UserRecord>().await {
                        Ok(body) => Ok(GetUserResponse { status, body }),
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
        user_id: &str,
        query: Query,
        request: PutUserRequest<T>,
    ) -> Result<GetUserResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, user_id);

        let http_client = reqwest::Client::new();
        let result = http_client
            .put(request_url)
            .query(&query.to_queries())
            .json(&request.user)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 200 {
                    match resp.json::<UserRecord>().await {
                        Ok(body) => Ok(GetUserResponse { status, body }),
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
        user_id: &str,
        query: Query,
    ) -> Result<DeleteUserResponse, ErrorResponse> {
        let request_url = format!("{}/{}", &self.url, user_id);

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
                        Ok(body) => Ok(DeleteUserResponse { status, body }),
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
