use crate::common::{app_cd, client_with_api_key, document_id};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn anyhow_works() -> anyhow::Result<()> {
    let app_cd = app_cd();
    let document_id = document_id();

    let query = Query::builder().app_cd(app_cd);

    let client = client_with_api_key();
    let resp = client.document.get(document_id, query).await?;
    assert_eq!(200u16, resp.status);

    Ok(())
}

#[tokio::test]
async fn clone_client_works() {
    let app_cd = app_cd();
    let document_id = document_id();

    let query = Query::builder().app_cd(app_cd);

    let client = client_with_api_key();
    let cloned_client = client.clone();

    let resp = client.document.get(document_id, query.clone()).await;
    assert_eq!(true, resp.is_ok());

    let resp = cloned_client.document.get(document_id, query).await;
    assert_eq!(true, resp.is_ok());
}
