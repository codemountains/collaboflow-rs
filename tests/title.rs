use crate::common::{client_with_api_key, client_with_password, title_id};
use collaboflow_rs::record::title::NewTitleRecord;
use collaboflow_rs::request::title::title_one::PutTitleRequest;
use collaboflow_rs::request::title::titles::PostTitlesRequest;
use collaboflow_rs::Query;
use serde::Deserialize;
use serde_json::json;

mod common;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Title {
    name: String,
    code: String,
}

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
async fn titles_with_fields_works() {
    let fields = vec!["name".to_string(), "code".to_string()];
    let query = Query::builder().fields(fields);
    let client = client_with_api_key();
    let resp = client.titles.get_with_fields::<Title>(query).await;
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

#[tokio::test]
async fn titles_create_update_delete_works() {
    let title = NewTitleRecord::new("TestTitle", "Test", "", 1);
    let request = PostTitlesRequest::new(title);
    let client = client_with_api_key();

    // create
    match client.titles.post(request).await {
        Ok(resp) => {
            let title_id = resp.body.id;
            let request = PutTitleRequest::new(json!({"level": 2}));

            // update
            match client.title.put(&title_id, Query::default(), request).await {
                Ok(resp) => {
                    let title_id = resp.body.id;

                    //delete
                    let resp = client.title.delete(&title_id, Query::default()).await;
                    assert_eq!(true, resp.is_ok());
                }
                Err(err) => panic!("{}", err),
            }
        }
        Err(err) => panic!("{}", err),
    }
}
