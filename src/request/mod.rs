use std::collections::HashMap;
use std::str;

pub struct HttpRequest {
    get_arguments: HashMap<String, String>,
    headers: HashMap<String, String>,
    method: String,
    post_arguments: HashMap<String, String>,
    request_uri: String
};

enum HttpRequest_Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE
}

impl HttpRequest {
    // TODO Implement this
    // This function answers whether request is a HTTP request or not
    pub fn matches(request: &[u8]) -> bool {
        if let Some(request) = str::from_utf8(buffer) {
            
        }
        false
    }

    // TODO Implement this
    pub fn from_tcp_stream(request: &[u8]) -> Result<HttpRequest, &'static str> {
        Err("Failed to parse stream")
    }
}

#[cfg(test)]
mod request_test {
    #[test]
    fn test_from_tcp_stream() {
        
    }

    #[test]
    fn test_matches() {
        
    }
}



