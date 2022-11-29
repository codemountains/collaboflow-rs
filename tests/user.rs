use crate::common::{client_with_api_key, client_with_password, user_unique_id};
use collaboflow_rs::Query;

mod common;

#[tokio::test]
async fn users_works() {
    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.users.get(query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.users.get(query).await;
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn user_works() {
    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.user.get(user_unique_id(), query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.user.get(user_unique_id(), query).await;
    assert_eq!(true, resp.is_ok());
}
