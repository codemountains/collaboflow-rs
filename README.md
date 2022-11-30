# Collaboflow REST API Client
[![Crates.io](https://img.shields.io/crates/v/collaboflow-rs.svg)](https://crates.io/crates/collaboflow-rs)
[![docs.rs](https://docs.rs/collaboflow-rs/badge.svg)](https://docs.rs/collaboflow-rs)
[![License](https://img.shields.io/github/license/codemountains/collaboflow-rs)](LICENSE)

A [Collaboflow REST API](http://docs.collaboflow.com/api-docs/) client for Rust language.

**This is not an official crate, just a hobby project.**

## Installation

### Requirements

- Rust 1.63+

### Importing

The driver is available on [crates.io](https://crates.io/crates/collaboflow-rs).

To use the driver in your application, simply add it to your project's `Cargo.toml`.

```toml
[dependencies]
collaboflow-rs = "0.0.4"
```

### Example Usage

```toml
[dependencies]
collaboflow-rs = "0.0.5"
tokio = "1.22.0"
```

```rust
use collaboflow_rs::{Authorization, CollaboflowClient, Query};

#[tokio::main]
async fn main() -> Result<(), ()> {
  let authorization = Authorization::with_api_key("User id", "API key");

  let client = CollaboflowClient::new("https://{Collaboflow url}/{Instance name}/api/index.cfm/v1/", authorization);
  
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
- [getDeterms](http://docs.collaboflow.com/api-docs/#/Document/getDeterms)
- [getDocumentContents](http://docs.collaboflow.com/api-docs/#/Document/getDocumentContents)

### MyStatus

- [getMyDeterms](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyDeterms)
- [getMyRequests](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyRequests)
- [getMyDrafts](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyDrafts)
- [getMyProcesses](http://docs.collaboflow.com/api-docs/#/MyStatus/getMyProcesses)

### Form

- [getForms](http://docs.collaboflow.com/api-docs/#/Form/getForms)
- [getFormParts](http://docs.collaboflow.com/api-docs/#/Form/getFormParts)
- [getFormSettingsPrints](http://docs.collaboflow.com/api-docs/#/Form/getFormSettingsPrints)

### User

- [getUsers](http://docs.collaboflow.com/api-docs/#/User/getUsers)
  - Query `fields` is not supported.
- [getUser](http://docs.collaboflow.com/api-docs/#/User/getUser)

### Group

- [getGroups](http://docs.collaboflow.com/api-docs/#/Group/getGroups)
  - Query `fields` is not supported.
- [getGroup](http://docs.collaboflow.com/api-docs/#/Group/getGroup)

### Title

- [getTitles](http://docs.collaboflow.com/api-docs/#/Title/getTitles)
  - Query `fields` is not supported.
- [getTitle](http://docs.collaboflow.com/api-docs/#/Title/getTitle)

## LICENSE

This project is licensed under the [MIT license](LICENSE).
