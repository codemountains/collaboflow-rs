use std::fmt;

/// Generate a query string.
///
/// ## Usage
///
/// ```rust
/// # use collaboflow_rs::{Query};
///
/// let query = Query::builder()
///     .app_cd(1)
///     .offset(0)
///     .limit(10);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Query {
    pub app_cd: Option<i32>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub current: Option<bool>,
    pub category_id: Option<i32>,
    pub detail: Option<bool>,
    pub key: Option<String>,
    pub fields: Option<Vec<String>>,
}

impl Query {
    pub fn builder() -> Self {
        Self {
            app_cd: None,
            offset: None,
            limit: None,
            current: None,
            category_id: None,
            detail: None,
            key: None,
            fields: None,
        }
    }

    pub fn to_queries(&self) -> Vec<(String, String)> {
        let mut queries: Vec<(String, String)> = Vec::new();
        if let Some(app_cd) = self.app_cd {
            queries.push(("app_cd".to_string(), app_cd.to_string()));
        }
        if let Some(offset) = self.offset {
            queries.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(limit) = self.limit {
            queries.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(current) = self.current {
            queries.push(("current".to_string(), current.to_string()));
        }
        if let Some(category_id) = self.category_id {
            queries.push(("category_id".to_string(), category_id.to_string()));
        }
        if let Some(detail) = self.detail {
            queries.push(("detail".to_string(), detail.to_string()));
        }
        if let Some(key) = &self.key {
            queries.push(("key".to_string(), key.to_string()));
        }

        queries
    }

    pub fn app_cd(mut self, app_cd: i32) -> Self {
        self.app_cd = Some(app_cd);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn current(mut self, current: bool) -> Self {
        self.current = Some(current);
        self
    }

    pub fn category_id(mut self, category_id: i32) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn detail(mut self, detail: bool) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }

    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut query_string = "".to_string();
        if let Some(app_cd) = self.app_cd {
            query_string = format!("app_cd={}", app_cd);
        }

        if let Some(offset) = self.offset {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("offset={}", offset).as_str();
        }

        if let Some(limit) = self.limit {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("limit={}", limit).as_str();
        }

        if let Some(current) = self.current {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("current={}", current).as_str();
        }

        if let Some(category_id) = self.category_id {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("category_id={}", category_id).as_str();
        }

        if let Some(detail) = self.detail {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("detail={}", detail).as_str();
        }

        if let Some(key) = &self.key {
            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("key={}", key).as_str();
        }

        if let Some(fields) = &self.fields {
            let mut fields_string = "".to_string();
            for field in fields.iter() {
                if !fields_string.is_empty() {
                    fields_string += ","
                }
                fields_string += field.as_str();
            }

            if !query_string.is_empty() {
                query_string += "&"
            }
            query_string += format!("fields={}", fields_string).as_str();
        }

        write!(f, "{}", query_string)
    }
}
