use crate::authorization::HEADER_KEY;
use crate::client::user::RESOURCE_V1_USERS;
use crate::record::user::UserRecord;
use crate::request::user::users::PostUsersRequest;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::user::users::{
    GetUsersResponse, GetUsersResponseBody, GetUsersResponseBodyWithFields,
    GetUsersResponseWithFields, PostUsersResponse,
};
use crate::Query;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Users {
    url: String,
    authorization_header: String,
}

impl Users {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_USERS,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(&self, query: Query) -> Result<GetUsersResponse, ErrorResponse> {
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
                    match resp.json::<GetUsersResponseBody>().await {
                        Ok(body) => Ok(GetUsersResponse { status, body }),
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
    ) -> Result<GetUsersResponseWithFields<T>, ErrorResponse> {
        let resp = self.get(query).await?;
        match serde_json::to_value(&resp.body.records) {
            Ok(v) => match serde_json::from_value::<Vec<T>>(v) {
                Ok(records) => Ok(GetUsersResponseWithFields {
                    status: resp.status,
                    body: GetUsersResponseBodyWithFields {
                        offset: resp.body.offset,
                        limit: resp.body.limit,
                        total_count: resp.body.total_count,
                        error: false,
                        records,
                    },
                }),
                Err(err) => {
                    let body = ErrorResponseBody {
                        error: true,
                        messages: vec![err.to_string()],
                    };
                    let error_response = ErrorResponse { status: 500, body };
                    Err(error_response)
                }
            },
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
        request: PostUsersRequest,
    ) -> Result<PostUsersResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .json(&request.user)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 201 {
                    match resp.json::<UserRecord>().await {
                        Ok(body) => Ok(PostUsersResponse { status, body }),
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
