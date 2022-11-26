pub mod authorization;
pub mod client;
pub mod query;
pub mod response;

pub use authorization::CollaboflowAuthorization;
pub use client::CollaboflowClient;
pub use query::Query;

#[cfg(test)]
mod tests {
    use crate::authorization::{AuthorizationType, CollaboflowAuthorization};
    use crate::client::CollaboflowClient;
    use crate::Query;
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
    const USER_UNIQUE_ID: &str = "USER_UNIQUE_ID";
    const GROUP_ID: &str = "GROUP_ID";
    const TITLE_ID: &str = "TITLE_ID";

    fn client_new(auth: AuthorizationType) -> CollaboflowClient {
        dotenv().ok();

        let authorization = match auth {
            AuthorizationType::ApiKey => CollaboflowAuthorization::with_api_key(
                env::var(USER_ID)
                    .expect(format!("{} is undefined.", USER_ID).as_str())
                    .as_str(),
                env::var(API_KEY)
                    .expect(format!("{} is undefined.", API_KEY).as_str())
                    .as_str(),
            ),
            AuthorizationType::Password => CollaboflowAuthorization::with_password(
                env::var(USER_ID)
                    .expect(format!("{} is undefined.", USER_ID).as_str())
                    .as_str(),
                env::var(USER_PASSWORD)
                    .expect(format!("{} is undefined.", USER_PASSWORD).as_str())
                    .as_str(),
            ),
        };

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

    fn user_unique_id() -> String {
        dotenv().ok();
        env::var(USER_UNIQUE_ID).expect(format!("{} is undefined.", USER_UNIQUE_ID).as_str())
    }

    fn group_id() -> String {
        dotenv().ok();
        env::var(GROUP_ID).expect(format!("{} is undefined.", GROUP_ID).as_str())
    }

    fn title_id() -> String {
        dotenv().ok();
        env::var(TITLE_ID).expect(format!("{} is undefined.", TITLE_ID).as_str())
    }

    #[tokio::test]
    async fn document_works() {
        let app_cd = 1;
        let document_id = document_id();

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client.document.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client.document.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn document_determs_works() {
        let app_cd = 1;
        let document_id = document_id();

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client.document_determs.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client.document_determs.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn document_contents_works() {
        let app_cd = 1;
        let document_id = document_id();

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client.document_contents.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client.document_contents.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn mydeterms_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client.mydeterms.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client.mydeterms.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn myrequests_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client.myrequests.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client.myrequests.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn mydrafts_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client.mydrafts.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client.mydrafts.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn myprocesses_works() {
        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_api_key();
        let resp = client.myprocesses.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let mut query_params = HashMap::new();
        query_params.insert("app_cd".to_string(), app_cd());

        let client = client_new_by_password();
        let resp = client.myprocesses.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn forms_works() {
        let app_cd = 1;

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client.forms.get(query).await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client.forms.get(query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn form_parts_works() {
        let app_cd = 1;
        let form_id = form_id();
        let form_version = form_version();

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client.form_parts.get(form_id, form_version, query).await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client.form_parts.get(form_id, form_version, query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn form_settings_prints_works() {
        let app_cd = 1;
        let form_id = form_id();
        let form_version = form_version();

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_api_key();
        let resp = client
            .form_settings_prints
            .get(form_id, form_version, query)
            .await;
        assert_eq!(true, resp.is_ok());

        let query = Query::builder().app_cd(app_cd);
        let client = client_new_by_password();
        let resp = client
            .form_settings_prints
            .get(form_id, form_version, query)
            .await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn users_works() {
        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.users.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.users.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn user_works() {
        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.user.get(user_unique_id(), query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.user.get(user_unique_id(), query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn groups_works() {
        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.groups.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.groups.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn group_works() {
        let group_id = group_id();

        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.group.get(&group_id, query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.group.get(&group_id, query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn titles_works() {
        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.titles.get(query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.titles.get(query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn title_works() {
        let title_id = title_id();

        let query_params = HashMap::new();

        let client = client_new_by_api_key();
        let resp = client.title.get(&title_id, query_params).await;
        assert_eq!(true, resp.is_ok());

        let query_params = HashMap::new();

        let client = client_new_by_password();
        let resp = client.title.get(&title_id, query_params).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn clone_client_works() {
        let document_id = document_id();

        let query = Query::builder().app_cd(1);

        let client = client_new_by_api_key();
        let cloned_client = client.clone();

        let resp = client.document.get(document_id, query.clone()).await;
        assert_eq!(true, resp.is_ok());

        let resp = cloned_client.document.get(document_id, query).await;
        assert_eq!(true, resp.is_ok());
    }

    #[tokio::test]
    async fn anyhow_works() -> anyhow::Result<()> {
        let document_id = document_id();

        let query = Query::builder().app_cd(1);

        let client = client_new_by_api_key();
        let resp = client.document.get(document_id, query).await?;
        assert_eq!(200u16, resp.status);

        Ok(())
    }

    #[test]
    fn query_works() {
        let fields = vec!["name".to_string(), "code".to_string()];

        let query = Query::builder()
            .app_cd(1)
            .offset(10)
            .limit(10)
            .current(true)
            .category_id(1)
            .detail(true)
            .fields(fields)
            .key("userid");
        println!("{}", query.to_string());
        assert_eq!(94, query.to_string().len());

        let empty_query = Query::builder();
        assert_eq!(true, empty_query.to_string().is_empty());
    }
}
