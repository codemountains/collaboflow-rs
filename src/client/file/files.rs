use crate::authorization::HEADER_KEY;
use crate::client::file::RESOURCE_V1_FILES;
use crate::record::file::FileRecord;
use crate::request::file::files::PostFilesRequest;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::file::files::{GetFileDownloadResponse, PostFilesResponse};
use reqwest::multipart;

#[derive(Debug, Clone)]
pub struct Files {
    url: String,
    authorization_header: String,
}

impl Files {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE_V1_FILES,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(&self, file_id: &str) -> Result<GetFileDownloadResponse, ErrorResponse> {
        let request_url = format!("{}/{}/download", &self.url, file_id);

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
                    match resp.bytes().await {
                        Ok(body) => {
                            // let body = bytes.to_vec();
                            Ok(GetFileDownloadResponse { status, body })
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
        request: PostFilesRequest,
    ) -> Result<PostFilesResponse, ErrorResponse> {
        let request_url = format!("{}", &self.url);

        let part = reqwest::multipart::Part::bytes(request.file_data)
            .file_name(request.file_name)
            .mime_str(&request.file_mine)
            .unwrap();
        let form = multipart::Form::new().part("file", part);

        let http_client = reqwest::Client::new();
        let result = http_client
            .post(request_url)
            .multipart(form)
            .header(HEADER_KEY, &self.authorization_header)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();

                if status == 201 {
                    match resp.json::<FileRecord>().await {
                        Ok(body) => Ok(PostFilesResponse { status, body }),
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
