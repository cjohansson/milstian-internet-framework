use std::collections::HashMap;

pub struct HttpRequest {
    get_arguments: HashMap<String, String>,
    headers: HashMap<String, String>,
    method: String,
    post_arguments: HashMap<String, String>,
    request_uri: String
};

impl HttpRequest {
    // TODO Implement this
    pub fn matches(&[u8]) -> bool {
        false
    }

    // TODO Implement this
    pub fn from_tcp_stream(&[u8]) -> Result<HttpRequest, &'static str> {
        Err("Failed to parse stream")
    }
}

#[cfg(test)]
mod request_test {
    #[test]
}



