use crate::authorization::HEADER_KEY;
use crate::client::document::RESOURCE_V1_DOCUMENTS;
use crate::response::document::document_contents::{
    GetDocumentContentsResponse, GetDocumentContentsResponseBody,
};
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::Query;

const NESTED_RESOURCE: &str = "contents";

#[derive(Debug, Clone)]
pub struct DocumentContents {
    url: String,
    authorization_header: String,
}

impl DocumentContents {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_DOCUMENTS,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        document_id: i32,
        query: Query,
    ) -> Result<GetDocumentContentsResponse, ErrorResponse> {
        let request_url = format!("{}/{}/{}", &self.url, document_id, NESTED_RESOURCE,);

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
                    match resp.json::<GetDocumentContentsResponseBody>().await {
                        Ok(body) => Ok(GetDocumentContentsResponse { status, body }),
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
