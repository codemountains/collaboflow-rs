use crate::common::{app_cd, client_with_api_key, client_with_password};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn mydeterms_works() {
    let app_cd = app_cd();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.mydeterms.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.mydeterms.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn myrequests_works() {
    let app_cd = app_cd();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.myrequests.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.myrequests.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn mydrafts_works() {
    let app_cd = app_cd();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.mydrafts.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.mydrafts.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn myprocesses_works() {
    let app_cd = app_cd();

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_api_key();
    let resp = client.myprocesses.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().app_cd(app_cd);
    let client = client_with_password();
    let resp = client.myprocesses.get(query).await;
    assert_eq!(true, resp.is_ok());
}
