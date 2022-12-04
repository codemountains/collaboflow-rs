use crate::common::{
    client_with_api_key, client_with_password, put_user_id, user_group_code, user_unique_id,
};
use collaboflow_rs::record::user::UserRecord;
use collaboflow_rs::request::user::user_one::PutUserRequest;
use collaboflow_rs::request::user::users::PostUsersRequest;
use collaboflow_rs::Query;
use serde::Serialize;
use serde_json::json;
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
    let user = UserRecord::new(
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
    let request = PostUsersRequest::new(user);

    let client = client_with_api_key();
    let resp = client.users.post(request).await;
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

#[tokio::test]
async fn user_put_works() {
    let user_id = put_user_id();

    let query = Query::builder().key("userid");
    let request = PutUserRequest::new(json!({"employee_code": "put_test_001"}));
    let client = client_with_api_key();
    let resp = client.user.put(&user_id, query, request).await;
    assert_eq!(true, resp.is_ok());

    let query = Query::builder().key("userid");
    let user = UpdateUser {
        extra1: "2022/12/01".to_string(),
        extra2: "09:30:45.123".to_string(),
        extra3: "collaboflow-rs test".to_string(),
    };
    let request = PutUserRequest::new(user);
    let client = client_with_password();
    let resp = client.user.put(&user_id, query, request).await;
    assert_eq!(true, resp.is_ok());
}

#[derive(Serialize)]
struct UpdateUser {
    extra1: String,
    extra2: String,
    extra3: String,
}
