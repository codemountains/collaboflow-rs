mod common;

use crate::common::{app_cd, client_with_api_key, client_with_password, document_id, processes_id};
use collaboflow_rs::request::document::documents::PostDocumentRequest;
use collaboflow_rs::Query;
use serde::Serialize;

#[tokio::test]
async fn document_post_works() {
    let app_cd = app_cd();
    let processes_id = processes_id();

    let document = NewDocument::new("API KEY", 1, 1000);
    let request = PostDocumentRequest::new(processes_id, None, None, None, app_cd, None, document);

    let client = client_with_api_key();
    let resp = client.documents.post(request).await;
    assert_eq!(true, resp.is_ok());

    let document = NewDocument::new("Password", 1, 1000);
    let request = PostDocumentRequest::new(processes_id, None, None, None, app_cd, None, document);

    let client = client_with_password();
    let resp = client.documents.post(request).await;
    assert_eq!(true, resp.is_ok());
}

#[derive(Serialize)]
struct NewDocument {
    fid1: String,
    fid2: i32,
    fid3: i32,
}

impl NewDocument {
    fn new(fid1: &str, fid2: i32, fid3: i32) -> Self {
        Self {
            fid1: fid1.to_string(),
            fid2,
            fid3,
        }
    }
}

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
