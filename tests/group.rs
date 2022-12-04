use crate::common::{client_with_api_key, client_with_password, group_id};
use collaboflow_rs::record::group::NewGroupRecord;
use collaboflow_rs::request::group::group_one::PutGroupRequest;
use collaboflow_rs::request::group::groups::PostGroupsRequest;
use collaboflow_rs::Query;
use serde_json::json;

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

#[tokio::test]
async fn group_create_update_delete_works() {
    let group = NewGroupRecord::new("TestGroup", "root", "Test G", "", "", 99);
    let request = PostGroupsRequest::new(group);
    let client = client_with_api_key();

    // create
    match client.groups.post(request).await {
        Ok(resp) => {
            let group_id = resp.body.id;
            let request = PutGroupRequest::new(json!({"display_name": "TEST GROUP"}));

            // update
            match client.group.put(&group_id, Query::default(), request).await {
                Ok(resp) => {
                    let group_id = resp.body.id;

                    // delete
                    let resp = client.group.delete(&group_id, Query::default()).await;
                    assert_eq!(true, resp.is_ok());
                }
                Err(err) => panic!("{}", err),
            }
        }
        Err(err) => panic!("{}", err),
    }
}
