# Collaboflow REST API Client

A Collaboflow REST API client for Rust language.

**This is not an official crate, just a hobby project.**

## Installation

### Requirements

- Rust 1.63+

### Importing

The driver is available on [crates.io](https://crates.io/). 

To use the driver in your application, simply add it to your project's `Cargo.toml`.

```toml
[dependencies]
collaboflow-rs = ""
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
- [getUser](http://docs.collaboflow.com/api-docs/#/User/getUser)

### Group

- [getGroups](http://docs.collaboflow.com/api-docs/#/Group/getGroups)
- [getGroup](http://docs.collaboflow.com/api-docs/#/Group/getGroup)

## LICENSE

This project is licensed under the [MIT license](LICENSE).
