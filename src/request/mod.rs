use std::collections::HashMap;
use std::str;

pub struct HttpRequest {
    get_arguments: HashMap<String, String>,
    headers: HashMap<String, String>,
    method: String,
    protocol: String,
    post_arguments: HashMap<String, String>,
    request_uri: String
}

enum HttpRequestMethod {
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

enum HttpRequestSection {
    REQUEST_LINE,
    HEADER_FIELDS,
    MESSAGE_BODY
}

impl HttpRequest {

    // TODO Implement this
    // TODO Implement this
    pub fn from_tcp_stream(request: &[u8]) -> Option<HttpRequest> {
        if let Ok(request) = str::from_utf8(request) {
            println!("Request as string: {}", request);
            if request.is_ascii() {
                println!("Request is ASCII");
                let mut section = HttpRequestSection::REQUEST_LINE;
                for line in request.lines() {
                    match section {
                        HttpRequestSection::REQUEST_LINE => {
                            // TODO: Do stuff here
                            section = HttpRequestSection::HEADER_FIELDS;
                        },
                        HttpRequestSection::HEADER_FIELDS => {
                            if line.trim().is_empty() {
                                section = HttpRequestSection::MESSAGE_BODY;
                            } else {
                                // TODO: Do stuff here
                            }
                        },
                        HttpRequestSection::MESSAGE_BODY => {
                            if !line.is_empty() {
                                // TODO: Do stuff here
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod request_test {
    use super::*;
    #[test]
    fn test_from_tcp_stream() {
        let response = HttpRequest::from_tcp_stream(
            b"GET /\r\n"
        );
        assert!(response.is_some());
    }
}



