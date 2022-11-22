use std::collections::HashMap;

pub fn query_string(query_params: HashMap<String, String>) -> String {
    let mut query_string = "".to_string();
    for (k, v) in &query_params {
        if query_string.is_empty() {
            query_string = format!("{}={}", k, v);
        } else {
            query_string += format!("&{}={}", k, v).as_str();
        }
    }
    query_string
}
