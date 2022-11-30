use collaboflow_rs::Query;

mod common;

#[test]
fn query_works() {
    let query = Query::builder()
        .app_cd(1)
        .offset(10)
        .limit(10)
        .current(true)
        .category_id(1)
        .detail(false)
        .key("userid");
    assert_eq!(78, query.to_string().len());

    let empty_query = Query::builder();
    assert_eq!(true, empty_query.to_string().is_empty());
}
