#![allow(dead_code)]

use collaboflow_rs::authorization::AuthorizationType;
use collaboflow_rs::{Authorization, CollaboflowClient};
use dotenv::dotenv;
use std::env;

const BASE_URL: &str = "BASE_URL";
const USER_ID: &str = "USER_ID";
const USER_PASSWORD: &str = "USER_PASSWORD";
const API_KEY: &str = "API_KEY";
const APP_CD: &str = "APP_CD";
const DOCUMENT_ID: &str = "DOCUMENT_ID";
const FORM_ID: &str = "FORM_ID";
const FORM_VERSION: &str = "FORM_VERSION";
const USER_UNIQUE_ID: &str = "USER_UNIQUE_ID";
const GROUP_ID: &str = "GROUP_ID";
const TITLE_ID: &str = "TITLE_ID";
const PROCESSES_ID: &str = "PROCESSES_ID";
const USER_GROUP_CODE: &str = "USER_GROUP_CODE";

pub fn client(auth_type: AuthorizationType) -> CollaboflowClient {
    dotenv().ok();

    let authorization = match auth_type {
        AuthorizationType::ApiKey => Authorization::with_api_key(
            env::var(USER_ID)
                .expect(format!("{} is undefined.", USER_ID).as_str())
                .as_str(),
            env::var(API_KEY)
                .expect(format!("{} is undefined.", API_KEY).as_str())
                .as_str(),
        ),
        AuthorizationType::Password => Authorization::with_password(
            env::var(USER_ID)
                .expect(format!("{} is undefined.", USER_ID).as_str())
                .as_str(),
            env::var(USER_PASSWORD)
                .expect(format!("{} is undefined.", USER_PASSWORD).as_str())
                .as_str(),
        ),
    };

    CollaboflowClient::new(
        env::var(BASE_URL)
            .expect(format!("{} is undefined.", BASE_URL).as_str())
            .as_str(),
        authorization,
    )
}

pub fn client_with_api_key() -> CollaboflowClient {
    client(AuthorizationType::ApiKey)
}

pub fn client_with_password() -> CollaboflowClient {
    client(AuthorizationType::Password)
}

pub fn app_cd() -> i32 {
    dotenv().ok();
    env::var(APP_CD)
        .expect(format!("{} is undefined.", APP_CD).as_str())
        .as_str()
        .parse::<i32>()
        .expect(format!("{} is not a number.", APP_CD).as_str())
}

pub fn document_id() -> i32 {
    dotenv().ok();
    env::var(DOCUMENT_ID)
        .expect(format!("{} is undefined.", DOCUMENT_ID).as_str())
        .as_str()
        .parse::<i32>()
        .expect(format!("{} is not a number.", DOCUMENT_ID).as_str())
}

pub fn form_id() -> i32 {
    dotenv().ok();
    env::var(FORM_ID)
        .expect(format!("{} is undefined.", FORM_ID).as_str())
        .as_str()
        .parse::<i32>()
        .expect(format!("{} is not a number.", FORM_ID).as_str())
}

pub fn form_version() -> i32 {
    dotenv().ok();
    env::var(FORM_VERSION)
        .expect(format!("{} is undefined.", FORM_VERSION).as_str())
        .as_str()
        .parse::<i32>()
        .expect(format!("{} is not a number.", FORM_VERSION).as_str())
}

pub fn user_unique_id() -> String {
    dotenv().ok();
    env::var(USER_UNIQUE_ID).expect(format!("{} is undefined.", USER_UNIQUE_ID).as_str())
}

pub fn group_id() -> String {
    dotenv().ok();
    env::var(GROUP_ID).expect(format!("{} is undefined.", GROUP_ID).as_str())
}

pub fn title_id() -> String {
    dotenv().ok();
    env::var(TITLE_ID).expect(format!("{} is undefined.", TITLE_ID).as_str())
}

pub fn processes_id() -> i32 {
    dotenv().ok();
    env::var(PROCESSES_ID)
        .expect(format!("{} is undefined.", PROCESSES_ID).as_str())
        .as_str()
        .parse::<i32>()
        .expect(format!("{} is not a number.", PROCESSES_ID).as_str())
}

pub fn user_group_code() -> String {
    dotenv().ok();
    env::var(USER_GROUP_CODE).expect(format!("{} is undefined.", USER_GROUP_CODE).as_str())
}
