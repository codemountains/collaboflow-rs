use crate::common::{client_with_api_key, client_with_password, user_group_code, user_unique_id};
use collaboflow_rs::request::user::users::PostUsersRequest;
use collaboflow_rs::Query;
use ulid::Ulid;

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
async fn users_post_works() {
    let test_user_id = Ulid::new().to_string();
    let user_group_code = user_group_code();
    let request = PostUsersRequest::new(
        &test_user_id,
        "test user",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        false,
        false,
        0,
        false,
        vec![user_group_code],
        vec![],
        "",
        "",
        "",
        "",
        "",
        "password",
    );

    let client = client_with_api_key();
    let resp = client.users.post(request).await;
    println!("{:?}", resp);
    assert_eq!(true, resp.is_ok());
}

#[tokio::test]
async fn user_works() {
    let user_id = user_unique_id();

    let query = Query::default();
    let client = client_with_api_key();
    let resp = client.user.get(&user_id, query).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::default();
    let client = client_with_password();
    let resp = client.user.get(&user_id, query).await;
    assert_eq!(true, resp.is_ok());
}
