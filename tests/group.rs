use crate::common::{client_with_api_key, client_with_password, group_id};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn groups_works() {
    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.groups.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.groups.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn group_works() {
    let group_id = group_id();

    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.group.get(&group_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.group.get(&group_id, query).await;
    assert_eq!(true, resp.is_ok());
}
