mod common;

use crate::common::{app_cd, client_with_api_key, client_with_password, document_id};
use collaboflow_rs::Query;

#[tokio::test]
async fn document_works() {
    let app_cd = app_cd();
    let document_id = document_id();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.document.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.document.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn document_contents_works() {
    let app_cd = app_cd();
    let document_id = document_id();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.document_contents.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.document_contents.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn document_determs_works() {
    let app_cd = app_cd();
    let document_id = document_id();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.document_determs.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.document_determs.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());
}
