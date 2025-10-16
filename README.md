# Collaboflow REST API Client

![archived](https://img.shields.io/badge/status-archived-red)
[![Crates.io](https://img.shields.io/crates/v/collaboflow-rs.svg)](https://crates.io/crates/collaboflow-rs)
[![msrv 1.65.0](https://img.shields.io/badge/msrv-1.65.0-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.65.0)
[![docs.rs](https://docs.rs/collaboflow-rs/badge.svg)](https://docs.rs/collaboflow-rs)
[![License](https://img.shields.io/github/license/codemountains/collaboflow-rs)](LICENSE)

> [!CAUTION]
> **This project is archived.**

A [Collaboflow REST API](http://docs.collaboflow.com/api-docs/) client for Rust language.

**This is not an official crate, just a hobby project.**

## Installation

### Requirements

- Rust 1.65.0+

### Importing

The driver is available on [crates.io](https://crates.io/crates/collaboflow-rs).

To use the driver in your application, simply add it to your project's `Cargo.toml`.

```toml
[dependencies]
collaboflow-rs = "1.0.3"
```

## Example Usage

```toml
[dependencies]
collaboflow-rs = "1.0.3"
tokio = "1.40.0"
```

```rust
use collaboflow_rs::{Authorization, CollaboflowClient, Query};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let authorization = Authorization::with_api_key("User id", "API key");

    let client = CollaboflowClient::new("https://{Collaboflow url}/{Instance name}/api/index.cfm", authorization);

    let query = Query::builder().app_cd(1); // Your app cd

    let document_id: i32 = 1; // Your document id
    let result = client.document.get(document_id, query).await;
    match result {
        Ok(resp) => {
            println!("{:?}", resp);
            Ok(())
        },
        Err(err) => {
            println!("{:?}", err);
            Err(())
        }
    }
}
```

## Support APIs

### Document

- [getDocumentOverview](http://docs.collaboflow.com/api-docs/#/Document/getDocumentOverview)
- [requestDocument](http://docs.collaboflow.com/api-docs/#/Document/requestDocument)
- [putDocumentStatus](http://docs.collaboflow.com/api-docs/#/Document/putDocumentStatus)
- [deleteDocument](http://docs.collaboflow.com/api-docs/#/Document/deleteDocument)
- [getDeterms](http://docs.collaboflow.com/api-docs/#/Document/getDeterms)
- [getDocumentContents](http://docs.collaboflow.com/api-docs/#/Document/getDocumentContents)
- [simulateDeterms](http://docs.collaboflow.com/api-docs/#/Document/simulateDeterms)
- [searchDocument](http://docs.collaboflow.com/api-docs/#/Document/searchDocument)

### MyStatus

- [getMyDeterms](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyDeterms)
- [getMyRequests](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyRequests)
- [getMyDrafts](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyDrafts)
- [getMyProcesses](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyProcesses)

### File

- [uploadFile](http://docs.collaboflow.com/api-docs/#/File/uploadFile)
- [downloadFile](http://docs.collaboflow.com/api-docs/#/File/downloadFile)

### Form

- [getForms](http://docs.collaboflow.com/api-docs/#/Form/getForms)
- [getFormParts](http://docs.collaboflow.com/api-docs/#/Form/getFormParts)
- [getFormSettingsPrints](http://docs.collaboflow.com/api-docs/#/Form/getFormSettingsPrints)

### User

- [getUsers](http://docs.collaboflow.com/api-docs/#/User/getUsers)
- [postUser](http://docs.collaboflow.com/api-docs/#/User/postUser)
- [getUser](http://docs.collaboflow.com/api-docs/#/User/getUser)
- [putUser](http://docs.collaboflow.com/api-docs/#/User/putUser)
- [deleteUser](http://docs.collaboflow.com/api-docs/#/User/deleteUser)

### Group

- [getGroups](http://docs.collaboflow.com/api-docs/#/Group/getGroups)
- [postGroup](http://docs.collaboflow.com/api-docs/#/Group/postGroup)
- [getGroup](http://docs.collaboflow.com/api-docs/#/Group/getGroup)
- [putGroup](http://docs.collaboflow.com/api-docs/#/Group/putGroup)
- [deleteGroup](http://docs.collaboflow.com/api-docs/#/Group/deleteGroup)

### Title

- [getTitles](http://docs.collaboflow.com/api-docs/#/Title/getTitles)
- [postTitle](http://docs.collaboflow.com/api-docs/#/Title/postTitle)
- [getTitle](http://docs.collaboflow.com/api-docs/#/Title/getTitle)
- [putTitle](http://docs.collaboflow.com/api-docs/#/Title/putTitle)
- [deleteTitle](http://docs.collaboflow.com/api-docs/#/Title/deleteTitle)

## LICENSE

This project is licensed under the [MIT license](LICENSE).
