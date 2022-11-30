use crate::common::{client_with_api_key, client_with_password, title_id};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn titles_works() {
    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.titles.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.titles.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn title_works() {
    let title_id = title_id();

    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.title.get(&title_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.title.get(&title_id, query).await;
    assert_eq!(true, resp.is_ok());
}
