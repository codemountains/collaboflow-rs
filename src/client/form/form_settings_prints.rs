use crate::client::HEADER_KEY;
use crate::query::query_string;
use crate::response::error::{ErrorResponse, ErrorResponseBody};
use crate::response::form::form_settings_prints::{
    GetFormSettingsPrintsResponse, GetFormSettingsPrintsResponseBody,
};
use std::collections::HashMap;

const RESOURCE: &str = "forms";
const NESTED_RESOURCE: &str = "versions";
const LAST_RESOURCE: &str = "settings/prints";

pub struct FormSettingsPrints {
    url: String,
    authorization_header: String,
}

impl FormSettingsPrints {
    pub fn new(url: &str, authorization_header: &str) -> Self {
        Self {
            url: url.to_string() + RESOURCE,
            authorization_header: authorization_header.to_string(),
        }
    }

    pub async fn get(
        &self,
        form_id: i32,
        form_version: i32,
        query_params: HashMap<String, String>,
    ) -> Result<GetFormSettingsPrintsResponse, ErrorResponse> {
        let request_url = format!(
            "{}/{}/{}/{}/{}?{}",
            &self.url,
            form_id,
            NESTED_RESOURCE,
            form_version,
            LAST_RESOURCE,
            query_string(query_params)
        );

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
                    match resp.json::<GetFormSettingsPrintsResponseBody>().await {
                        Ok(body) => Ok(GetFormSettingsPrintsResponse { status, body }),
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
