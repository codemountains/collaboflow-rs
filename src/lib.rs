mod authorization;
mod client;
pub mod response;

pub use authorization::AuthorizationType;
pub use authorization::CollaboflowAuthorization;
pub use client::CollaboflowClient;

#[cfg(test)]
mod tests {
    use crate::authorization::{AuthorizationType, CollaboflowAuthorization};
    use crate::client::CollaboflowClient;
    use dotenv::dotenv;
    use std::collections::HashMap;
    use std::env;

    const BASE_URL: &str = "BASE_URL";
    const USER_ID: &str = "USER_ID";
    const USER_PASSWORD: &str = "USER_PASSWORD";
    const API_KEY: &str = "API_KEY";
    const APP_CD: &str = "APP_CD";
    const DOCUMENT_ID: &str = "DOCUMENT_ID";
    const FORM_ID: &str = "FORM_ID";
    const FORM_VERSION: &str = "FORM_VERSION";

    fn client_new(auth: AuthorizationType) -> CollaboflowClient {
        dotenv().ok();

        let value_key = match auth {
            AuthorizationType::Password => USER_PASSWORD,
            AuthorizationType::ApiKey => API_KEY,
        };

        let authorization = CollaboflowAuthorization::new(
            auth,
            env::var(USER_ID)
                .expect(format!("{} is undefined.", USER_ID).as_str())
                .as_str(),
            env::var(value_key)
                .expect(format!("{} is undefined.", value_key).as_str())
                .as_str(),
        );

        CollaboflowClient::new(
            env::var(BASE_URL)
                .expect(format!("{} is undefined.", BASE_URL).as_str())
                .as_str(),
            authorization,
        )
    }

    fn client_new_by_api_key() -> CollaboflowClient {
        client_new(AuthorizationType::ApiKey)
    }

    fn client_new_by_password() -> CollaboflowClient {
        client_new(AuthorizationType::Password)
    }

    fn app_cd() -> String {
        dotenv().ok();
        env::var(APP_CD).expect(format!("{} is undefined.", APP_CD).as_str())
    }

    fn document_id() -> i32 {
        dotenv().ok();
        env::var(DOCUMENT_ID)
            .expect(format!("{} is undefined.", DOCUMENT_ID).as_str())
            .as_str()
            .parse::<i32>()
            .expect(format!("{} is not a number.", DOCUMENT_ID).as_str())
    }

    fn form_id() -> i32 {
        dotenv().ok();
        env::var(FORM_ID)
            .expect(format!("{} is undefined.", FORM_ID).as_str())
            .as_str()
            .parse::<i32>()
            .expect(format!("{} is not a number.", FORM_ID).as_str())
    }

    fn form_version() -> i32 {
        dotenv().ok();
        env::var(FORM_VERSION)
            .expect(format!("{} is undefined.", FORM_VERSION).as_str())
            .as_str()
            .parse::<i32>()
            .expect(format!("{} is not a number.", FORM_VERSION).as_str())
    }

    #[tokio::test]
    async fn documents_works() {
        let document_id = document_id();

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.documents.get(document_id, query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.documents.get(document_id, query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn document_determs_works() {
        let document_id = document_id();

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.document_determs.get(document_id, query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.document_determs.get(document_id, query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn document_contents_works() {
        let document_id = document_id();

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client
            .document_contents
            .get(document_id, query_params)
            .await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client
            .document_contents
            .get(document_id, query_params)
            .await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn mydeterms_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.mydeterms.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.mydeterms.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn myrequests_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.myrequests.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.myrequests.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn mydrafts_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.mydrafts.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.mydrafts.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn myprocesses_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.myprocesses.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.myprocesses.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn forms_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_api_key();
        let resp = client.forms.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), "1".to_string());

        let client = client_new_by_password();
        let resp = client.forms.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn form_parts_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client.form_parts.get(32, 1, query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client.form_parts.get(32, 1, query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn form_settings_prints_works() {
        let form_id = form_id();
        let form_version = form_version();

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client
            .form_settings_prints
            .get(form_id, form_version, query_params)
            .await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client
            .form_settings_prints
            .get(form_id, form_version, query_params)
            .await;
        assert_eq!(true, resp.is_ok());
    }
}
