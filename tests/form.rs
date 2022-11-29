use crate::common::{app_cd, client_with_api_key, client_with_password, form_id, form_version};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn forms_works() {
    let app_cd = app_cd();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.forms.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.forms.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn form_parts_works() {
    let app_cd = app_cd();
    let form_id = form_id();
    let form_version = form_version();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.form_parts.get(form_id, form_version, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.form_parts.get(form_id, form_version, query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn form_settings_prints_works() {
    let app_cd = app_cd();
    let form_id = form_id();
    let form_version = form_version();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client
        .form_settings_prints
        .get(form_id, form_version, query)
        .await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client
        .form_settings_prints
        .get(form_id, form_version, query)
        .await;
    assert_eq!(true, resp.is_ok());
}
