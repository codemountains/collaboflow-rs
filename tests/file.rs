use crate::common::{client_with_api_key, client_with_password};
use collaboflow_rs::request::file::files::PostFilesRequest;
use std::fs;

mod common;

#[tokio::test]
async fn text_file_upload_and_download_with_api_key_works() {
    let file_data =
        fs::read("./tests/sample_files/test.txt").expect("Failed to read the file 'test.txt'.");
    let request =
        PostFilesRequest::new("test.txt".to_string(), "text/plain".to_string(), file_data);

    let client = client_with_api_key();
    let resp = client.files.post(request).await;
    assert_eq!(true, resp.is_ok());

    if let Ok(result) = resp {
        let resp = client.files.get(result.body.id.as_str()).await;
        assert_eq!(true, resp.is_ok());
    }
}

#[tokio::test]
async fn text_file_upload_and_download_with_password_works() {
    let file_data =
        fs::read("./tests/sample_files/test.txt").expect("Failed to read the file 'test.txt'.");
    let request =
        PostFilesRequest::new("test.txt".to_string(), "text/plain".to_string(), file_data);

    let client = client_with_password();
    let resp = client.files.post(request).await;
    assert_eq!(true, resp.is_ok());

    if let Ok(result) = resp {
        let resp = client.files.get(result.body.id.as_str()).await;
        assert_eq!(true, resp.is_ok());
    }
}

#[tokio::test]
async fn image_file_upload_and_download_with_api_key_works() {
    let file_data =
        fs::read("./tests/sample_files/test.png").expect("Failed to read the file 'test.png'.");
    let request = PostFilesRequest::new("test.png".to_string(), "image/png".to_string(), file_data);

    let client = client_with_api_key();
    let resp = client.files.post(request).await;
    assert_eq!(true, resp.is_ok());

    if let Ok(result) = resp {
        let resp = client.files.get(result.body.id.as_str()).await;
        assert_eq!(true, resp.is_ok());
    }
}

#[tokio::test]
async fn image_file_upload_and_download_with_password_works() {
    let file_data =
        fs::read("./tests/sample_files/test.png").expect("Failed to read the file 'test.png'.");
    let request = PostFilesRequest::new("test.png".to_string(), "image/png".to_string(), file_data);

    let client = client_with_password();
    let resp = client.files.post(request).await;
    assert_eq!(true, resp.is_ok());

    if let Ok(result) = resp {
        let resp = client.files.get(result.body.id.as_str()).await;
        assert_eq!(true, resp.is_ok());
    }
}
