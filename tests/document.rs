mod common;

use crate::common::{app_cd, client_with_api_key, client_with_password, document_id, processes_id};
use collaboflow_rs::request::document::document_one::PutDocumentStatusRequest;
use collaboflow_rs::request::document::documents::PostDocumentRequest;
use collaboflow_rs::Query;
use serde::Serialize;
use serde_json::json;

#[tokio::test]
async fn document_post_works() {
    let app_cd = app_cd();
    let processes_id = processes_id();

    let document = NewDocument::new("API KEY", 1, 1000);
    let request = PostDocumentRequest::new(processes_id, None, None, None, app_cd, None, document);

    let client = client_with_api_key();
    let resp = client.documents.post(request).await;
    assert_eq!(true, resp.is_ok());

    let document = json!({
        "fid1": "Password",
        "fid2": 999,
        "fid3": 54321,
    });
    let request = PostDocumentRequest::new(processes_id, None, None, None, app_cd, None, document);

    let client = client_with_password();
    let resp = client.documents.post(request).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn document_get_works() {
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
async fn document_put_works() {
    let app_cd = app_cd();
    let processes_id = processes_id();

    let document = NewDocument::new("draft to request", 1, 1000);
    let request = PostDocumentRequest::new(
        processes_id,
        None,
        None,
        Some("draft"),
        app_cd,
        None,
        document,
    );

    let client = client_with_api_key();
    match client.documents.post(request).await {
        Ok(resp) => {
            let document_id = resp.body.document_id;
            let query = Query::builder().app_cd(app_cd);
            let request = PutDocumentStatusRequest::<NewDocument>::new(
                "request", None, None, None, None, None,
            );
            let resp = client.document.put(document_id, query, request).await;
            assert_eq!(true, resp.is_ok());
        }
        Err(err) => assert_ne!(200, err.status),
    }
}

#[tokio::test]
async fn document_delete_works() {
    let app_cd = app_cd();
    let processes_id = processes_id();

    let document = NewDocument::new("draft to request", 1, 1000);
    let request = PostDocumentRequest::new(
        processes_id,
        None,
        None,
        Some("draft"),
        app_cd,
        None,
        document,
    );

    let client = client_with_api_key();
    match client.documents.post(request).await {
        Ok(resp) => {
            let document_id = resp.body.document_id;
            let query = Query::builder().app_cd(app_cd);
            let resp = client.document.delete(document_id, query).await;
            assert_eq!(true, resp.is_ok());
        }
        Err(err) => assert_ne!(200, err.status),
    }
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
