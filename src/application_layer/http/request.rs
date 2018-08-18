use std::collections::HashMap;
use std::str;

// TODO Should support parsing of different message body encodings
// TODO Support multi-part message bodies and gzip
// TODO Support keep-alive
// TODO Support TLS? Maybe that is at the level below?

#[derive(Debug)]
pub struct Message {
    pub body: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub request_line: Line,
}

#[derive(Debug)]
pub struct Line {
    pub method: Method,
    pub protocol: Protocol,
    pub request_uri: String,
    pub request_uri_base: String,
    pub query_arguments: HashMap<String, String>,
    pub query_string: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Invalid,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Protocol {
    Invalid,
    OneDotZero,
    OneDotOne,
    TwoDotZero,
    ZeroDotNine,
}

enum Section {
    Line,
    HeaderFields,
    MessageBody,
}

#[derive(Debug, Eq, PartialEq)]
enum SettingValence {
    Optional,
    No,
    Yes,
}

impl Message {
    fn method_has_request_body(method: &Method) -> SettingValence {
        match method {
            Method::Connect => SettingValence::Yes,
            Method::Delete => SettingValence::No,
            Method::Get => SettingValence::Optional,
            Method::Head => SettingValence::No,
            Method::Options => SettingValence::Optional,
            Method::Patch => SettingValence::Yes,
            Method::Post => SettingValence::Yes,
            Method::Put => SettingValence::Yes,
            Method::Trace => SettingValence::Yes,
            Method::Invalid => SettingValence::Optional,
        }
    }

    fn _method_has_response_body(method: &Method) -> bool {
        match method {
            Method::Connect => true,
            Method::Delete => true,
            Method::Get => true,
            Method::Head => false,
            Method::Options => true,
            Method::Patch => true,
            Method::Post => true,
            Method::Put => true,
            Method::Trace => true,
            Method::Invalid => true,
        }
    }

    fn _method_is_safe(method: &Method) -> bool {
        match method {
            Method::Connect => false,
            Method::Delete => false,
            Method::Get => true,
            Method::Head => true,
            Method::Options => true,
            Method::Patch => false,
            Method::Post => false,
            Method::Put => false,
            Method::Trace => true,
            Method::Invalid => true,
        }
    }

    fn _method_is_idempotent(method: &Method) -> bool {
        match method {
            Method::Connect => false,
            Method::Delete => true,
            Method::Get => true,
            Method::Head => true,
            Method::Options => true,
            Method::Patch => false,
            Method::Post => false,
            Method::Put => true,
            Method::Trace => true,
            Method::Invalid => true,
        }
    }

    fn _method_is_cacheable(method: &Method) -> bool {
        match method {
            Method::Connect => false,
            Method::Delete => false,
            Method::Get => true,
            Method::Head => true,
            Method::Options => false,
            Method::Patch => false,
            Method::Post => true,
            Method::Put => false,
            Method::Trace => false,
            Method::Invalid => false,
        }
    }

    fn get_query_args_from_string(subject: &str) -> Option<HashMap<String, String>> {
        let mut args: HashMap<String, String> = HashMap::new();
        if !subject.is_empty() {
            let subject_arguments: Vec<&str> = subject.split("&").collect();
            for item in subject_arguments {
                let query_arg: Vec<&str> = item.split("=").collect();
                if query_arg.len() == 2 {
                    args.insert(query_arg.get(0)?.to_string(), query_arg.get(1)?.to_string());
                } else {
                    args.insert(query_arg.get(0)?.to_string(), String::from("1"));
                }
            }
        }
        if args.len() > 0 {
            return Some(args);
        }
        None
    }

    pub fn get_protocol_text(protocol: &Protocol) -> String {
        match protocol {
            Protocol::ZeroDotNine => String::from("HTTP/0.9"),
            Protocol::OneDotZero => String::from("HTTP/1.0"),
            Protocol::OneDotOne => String::from("HTTP/1.1"),
            Protocol::TwoDotZero => String::from("HTTP/2.0"),
            Protocol::Invalid => String::from("INVALID"),
        }
    }

    // TODO This associated function should parse body based on content encoding
    pub fn get_message_body(body: &str) -> Option<HashMap<String, String>> {
        Message::get_query_args_from_string(body)
    }

    pub fn get_header_field(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        if !line.is_empty() {
            let parts: Vec<&str> = line.splitn(2, ":").collect();
            if parts.len() == 2 {
                let header_key = parts.get(0)?.trim().to_string();
                let header_value = parts.get(1)?.trim().to_string();
                return Some((header_key, header_value));
            }
        }
        None
    }

    pub fn get_request_line(line: &str) -> Option<Line> {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 3 {
            let method = match parts.get(0)?.as_ref() {
                "CONNECT" => Method::Connect,
                "DELETE" => Method::Delete,
                "GET" => Method::Get,
                "HEAD" => Method::Head,
                "OPTIONS" => Method::Options,
                "PATCH" => Method::Patch,
                "PUT" => Method::Put,
                "POST" => Method::Post,
                "TRACE" => Method::Trace,
                __ => Method::Invalid,
            };

            let request_uri = parts.get(1)?.to_string();
            let request_uri_copy = request_uri.clone();
            let mut request_uri_base = request_uri.clone();
            let mut query_string = String::new();
            let mut query_arguments: HashMap<String, String> = HashMap::new();
            let uri_parts: Vec<&str> = request_uri_copy.splitn(2, "?").collect();
            if uri_parts.len() == 2 {
                request_uri_base = uri_parts.get(0)?.to_string();
                query_string = uri_parts.get(1)?.to_string();
                if let Some(query_args) = Message::get_query_args_from_string(&query_string) {
                    query_arguments = query_args;
                }
            }

            let protocol = match parts.get(2)?.as_ref() {
                "HTTP/0.9" => Protocol::ZeroDotNine,
                "HTTP/1.0" => Protocol::OneDotZero,
                "HTTP/1.1" => Protocol::OneDotOne,
                "HTTP/2.0" => Protocol::TwoDotZero,
                _ => Protocol::Invalid,
            };

            if method != Method::Invalid && protocol != Protocol::Invalid {
                return Some(Line {
                    method,
                    protocol,
                    request_uri,
                    request_uri_base,
                    query_arguments,
                    query_string,
                });
            }
        } else if parts.len() == 1 {
            // Add support a request line containing only the path name is accepted by servers to maintain compatibility with  clients before the HTTP/1.0 specification
            let method = Method::Get;
            let request_uri = parts.get(0)?.trim_matches(char::from(0)).to_string();
            if !request_uri.is_empty() {
                let protocol = Protocol::ZeroDotNine;

                let request_uri_copy = request_uri.clone();
                let mut request_uri_base = request_uri.clone();
                let mut query_string = String::new();
                let mut query_arguments: HashMap<String, String> = HashMap::new();

                let uri_parts: Vec<&str> = request_uri_copy.splitn(2, "?").collect();
                if uri_parts.len() == 2 {
                    request_uri_base = uri_parts.get(0)?.to_string();
                    query_string = uri_parts.get(1)?.to_string();
                    if let Some(query_args) = Message::get_query_args_from_string(&query_string) {
                        query_arguments = query_args;
                    }
                }

                return Some(Line {
                    method,
                    protocol,
                    request_uri,
                    request_uri_base,
                    query_arguments,
                    query_string,
                });
            }
        }
        None
    }

    pub fn from_tcp_stream(request: &[u8]) -> Option<Message> {
        if let Ok(mut request) = str::from_utf8(request) {
            if request.is_ascii() {
                // Trim null bytes
                request = request.trim_matches(char::from(0));
                // println!("request: {}", request);
                let mut headers: HashMap<String, String> = HashMap::new();
                let mut body: HashMap<String, String> = HashMap::new();
                let mut request_line: Line = Line {
                    method: Method::Invalid,
                    protocol: Protocol::Invalid,
                    request_uri: String::new(),
                    request_uri_base: String::new(),
                    query_arguments: HashMap::new(),
                    query_string: String::new(),
                };
                let mut section = Section::Line;
                for mut line in request.lines() {
                    match section {
                        Section::Line => {
                            request_line = Message::get_request_line(line)?;
                            section = Section::HeaderFields;
                        }
                        Section::HeaderFields => {
                            if line.trim().is_empty() {
                                section = Section::MessageBody;
                            } else {
                                let (header_key, header_value) = Message::get_header_field(line)?;
                                headers.insert(header_key, header_value);
                            }
                        }
                        Section::MessageBody => {
                            if line.is_empty() {
                                break;
                            } else if Message::method_has_request_body(&request_line.method)
                                != SettingValence::No
                            {
                                if let Some(body_args) = Message::get_message_body(line) {
                                    body = body_args;
                                }
                            }
                        }
                    }
                }
                if request_line.method != Method::Invalid
                    && request_line.protocol != Protocol::Invalid
                {
                    return Some(Message {
                        body,
                        headers,
                        request_line,
                    });
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
    fn test_get_message_body() {
        let response = Message::get_message_body("random=abc&hej=def&def");
        assert!(response.is_some());

        let response_unwrapped = response.unwrap();
        assert_eq!(
            response_unwrapped
                .get(&"random".to_string())
                .unwrap()
                .to_string(),
            "abc".to_string()
        );
        assert_eq!(
            response_unwrapped
                .get(&"hej".to_string())
                .unwrap()
                .to_string(),
            "def".to_string()
        );
        assert_eq!(
            response_unwrapped
                .get(&"def".to_string())
                .unwrap()
                .to_string(),
            "1".to_string()
        );
        assert!(response_unwrapped.get(&"defs".to_string()).is_none());

        let response = Message::get_message_body("");
        assert!(response.is_none());
    }

    #[test]
    fn test_get_header_field() {
        let response = Message::get_header_field(
            "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0\r\n",
        );
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(key, "User-Agent".to_string());
        assert_eq!(
            value,
            "Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0".to_string()
        );

        let response = Message::get_header_field("Cache-Control: no-cache \r\n");
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(key, "Cache-Control".to_string());
        assert_eq!(value, "no-cache".to_string());

        let response = Message::get_header_field("Just various text here\r\n");
        assert!(response.is_none());

        let response = Message::get_header_field("");
        assert!(response.is_none());
    }

    #[test]
    fn test_get_request_line() {
        let response = Message::get_request_line("POST /random?abc=test HTTP/0.9\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, Method::Post);
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/random?abc=test")
        );
        assert_eq!(response_unpacked.request_uri_base, String::from("/random"));
        assert_eq!(response_unpacked.query_string, String::from("abc=test"));
        assert_eq!(
            response_unpacked
                .query_arguments
                .get(&"abc".to_string())
                .unwrap()
                .to_string(),
            String::from("test")
        );
        assert_eq!(response_unpacked.protocol, Protocol::ZeroDotNine);

        let response = Message::get_request_line("GET / HTTP/1.0\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, Method::Get);
        assert_eq!(response_unpacked.request_uri, String::from("/"));
        assert_eq!(response_unpacked.request_uri_base, String::from("/"));
        assert_eq!(response_unpacked.query_string, String::from(""));
        assert_eq!(response_unpacked.protocol, Protocol::OneDotZero);

        let response = Message::get_request_line("HEAD /moradish.html?test&abc=def HTTP/1.1\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, Method::Head);
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/moradish.html?test&abc=def")
        );
        assert_eq!(
            response_unpacked.request_uri_base,
            String::from("/moradish.html")
        );
        assert_eq!(response_unpacked.query_string, String::from("test&abc=def"));
        assert_eq!(
            response_unpacked
                .query_arguments
                .get(&"test".to_string())
                .unwrap()
                .to_string(),
            String::from("1")
        );
        assert_eq!(
            response_unpacked
                .query_arguments
                .get(&"abc".to_string())
                .unwrap()
                .to_string(),
            String::from("def")
        );
        assert_eq!(response_unpacked.protocol, Protocol::OneDotOne);

        let response = Message::get_request_line("OPTIONS /random/random2.txt HTTP/2.0\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, Method::Options);
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/random/random2.txt")
        );
        assert_eq!(response_unpacked.protocol, Protocol::TwoDotZero);

        let response = Message::get_request_line("GET / HTTP/2.2\r\n");
        assert!(response.is_none());
    }

    #[test]
    fn from_tcp_stream() {
        // GET request with no headers or body
        let response = Message::from_tcp_stream(b"GET / HTTP/2.0\r\n");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(response_unwrapped.request_line.method, Method::Get);
        assert_eq!(response_unwrapped.request_line.request_uri, "/".to_string());
        assert_eq!(
            response_unwrapped.request_line.protocol,
            Protocol::TwoDotZero
        );

        // POST request with random header and null bytes
        let mut request: Vec<u8> =
            b"POST /random HTTP/1.0\r\nAgent: Random browser\r\n\r\ntest=abc".to_vec();
        request.push(0);
        request.push(0);
        let response = Message::from_tcp_stream(&request);
        assert!(response.is_some());
        assert_eq!(
            "/random".to_string(),
            response.unwrap().request_line.request_uri
        );

        // POST request with random header
        let response =
            Message::from_tcp_stream(b"POST / HTTP/1.0\r\nAgent: Random browser\r\n\r\ntest=abc");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(response_unwrapped.request_line.method, Method::Post);
        assert_eq!(
            response_unwrapped.request_line.protocol,
            Protocol::OneDotZero
        );
        assert_eq!(
            response_unwrapped
                .headers
                .get(&"Agent".to_string())
                .unwrap()
                .to_string(),
            "Random browser".to_string()
        );
        assert_eq!(
            response_unwrapped
                .body
                .get(&"test".to_string())
                .unwrap()
                .to_string(),
            "abc".to_string()
        );

        // Two invalid  requests
        let response = Message::from_tcp_stream(b"RANDOM /stuff HTTP/2.5\r\n");
        assert!(response.is_none());
        let response = Message::from_tcp_stream(b"");
        assert!(response.is_none());

        // Get requests should get their message body parsed
        let response = Message::from_tcp_stream(b"GET / HTTP/2.0\r\n\r\nabc=123");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(
            response_unwrapped
                .body
                .get(&"abc".to_string())
                .unwrap()
                .to_string(),
            "123".to_string()
        );

        // HEAD requests should not get their message body parsed
        let response = Message::from_tcp_stream(b"HEAD / HTTP/2.0\r\n\r\nabc=123");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert!(response_unwrapped.body.get(&"abc".to_string()).is_none());

        let response = Message::from_tcp_stream(b"html/index.html\r\n");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(response_unwrapped.request_line.method, Method::Get);
        assert_eq!(
            response_unwrapped.request_line.request_uri,
            "html/index.html".to_string()
        );
        assert_eq!(
            response_unwrapped.request_line.protocol,
            Protocol::ZeroDotNine
        );

        let response = Message::from_tcp_stream(&[0; 100]);
        assert!(response.is_none());
    }

}
