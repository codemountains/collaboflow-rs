use crate::authorization::HEADER_KEY;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::mystatus::mydeterms::{GetMyDetermsResponse, GetMyDetermsResponseBody};
use crate::Query;

const RESOURCE: &str = "/v1/mydeterms";

#[derive(Debug, Clone)]
pub struct MyDeterms {
    url: String,
    authorization_header: String,
}

impl MyDeterms {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(&self, query: Query) -> Result<GetMyDetermsResponse, ErrorResponse> {
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
                    match resp.json::<GetMyDetermsResponseBody>().await {
                        Ok(body) => Ok(GetMyDetermsResponse { status, body }),
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
