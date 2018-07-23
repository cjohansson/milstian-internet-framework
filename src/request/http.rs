use std::collections::HashMap;
use std::str;

// TODO Should support parsing of different message body encodings
// TODO Support multi-part message bodies and gzip
// TODO Support keep-alive
// TODO Support TLS?

#[derive(Debug)]
pub struct RequestMessage {
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
    request_line: RequestLine,
}

#[derive(Debug)]
pub struct RequestLine {
    method: RequestMethod,
    protocol: RequestProtocol,
    request_uri: String,
    request_uri_base: String,
    query_arguments: HashMap<String, String>,
    query_string: String,
}

#[derive(Debug, Eq, PartialEq)]
enum RequestMethod {
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
enum RequestProtocol {
    Invalid,
    OneDotZero,
    OneDotOne,
    TwoDotZero,
    ZeroDotNine,
}

enum RequestSection {
    RequestLine,
    HeaderFields,
    MessageBody,
}

#[derive(Debug, Eq, PartialEq)]
enum SettingValence {
    Optional,
    No,
    Yes,
}

impl RequestMessage {
    fn method_has_request_body(method: &RequestMethod) -> SettingValence {
        match method {
            RequestMethod::Connect => SettingValence::Yes,
            RequestMethod::Delete => SettingValence::No,
            RequestMethod::Get => SettingValence::Optional,
            RequestMethod::Head => SettingValence::No,
            RequestMethod::Options => SettingValence::Optional,
            RequestMethod::Patch => SettingValence::Yes,
            RequestMethod::Post => SettingValence::Yes,
            RequestMethod::Put => SettingValence::Yes,
            RequestMethod::Trace => SettingValence::Yes,
            RequestMethod::Invalid => SettingValence::Optional,
        }
    }

    fn _method_has_response_body(method: &RequestMethod) -> bool {
        match method {
            RequestMethod::Connect => true,
            RequestMethod::Delete => true,
            RequestMethod::Get => true,
            RequestMethod::Head => false,
            RequestMethod::Options => true,
            RequestMethod::Patch => true,
            RequestMethod::Post => true,
            RequestMethod::Put => true,
            RequestMethod::Trace => true,
            RequestMethod::Invalid => true,
        }
    }

    fn _method_is_safe(method: &RequestMethod) -> bool {
        match method {
            RequestMethod::Connect => false,
            RequestMethod::Delete => false,
            RequestMethod::Get => true,
            RequestMethod::Head => true,
            RequestMethod::Options => true,
            RequestMethod::Patch => false,
            RequestMethod::Post => false,
            RequestMethod::Put => false,
            RequestMethod::Trace => true,
            RequestMethod::Invalid => true,
        }
    }

    fn _method_is_idempotent(method: &RequestMethod) -> bool {
        match method {
            RequestMethod::Connect => false,
            RequestMethod::Delete => true,
            RequestMethod::Get => true,
            RequestMethod::Head => true,
            RequestMethod::Options => true,
            RequestMethod::Patch => false,
            RequestMethod::Post => false,
            RequestMethod::Put => true,
            RequestMethod::Trace => true,
            RequestMethod::Invalid => true,
        }
    }

    fn _method_is_cacheable(method: &RequestMethod) -> bool {
        match method {
            RequestMethod::Connect => false,
            RequestMethod::Delete => false,
            RequestMethod::Get => true,
            RequestMethod::Head => true,
            RequestMethod::Options => false,
            RequestMethod::Patch => false,
            RequestMethod::Post => true,
            RequestMethod::Put => false,
            RequestMethod::Trace => false,
            RequestMethod::Invalid => false,
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

    // TODO This associated function should parse body based on encoding
    pub fn get_message_body(body: &str) -> Option<HashMap<String, String>> {
        RequestMessage::get_query_args_from_string(body)
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

    pub fn get_request_line(line: &str) -> Option<RequestLine> {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 3 {
            let method = match parts.get(0)?.as_ref() {
                "CONNECT" => RequestMethod::Connect,
                "DELETE" => RequestMethod::Delete,
                "GET" => RequestMethod::Get,
                "HEAD" => RequestMethod::Head,
                "OPTIONS" => RequestMethod::Options,
                "PATCH" => RequestMethod::Patch,
                "PUT" => RequestMethod::Put,
                "POST" => RequestMethod::Post,
                "TRACE" => RequestMethod::Trace,
                __ => RequestMethod::Invalid,
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
                if let Some(query_args) = RequestMessage::get_query_args_from_string(&query_string) {
                    query_arguments = query_args;
                }
            }

            let protocol = match parts.get(2)?.as_ref() {
                "/0.9" => RequestProtocol::ZeroDotNine,
                "/1.0" => RequestProtocol::OneDotZero,
                "/1.1" => RequestProtocol::OneDotOne,
                "/2.0" => RequestProtocol::TwoDotZero,
                _ => RequestProtocol::Invalid,
            };

            if method != RequestMethod::Invalid && protocol != RequestProtocol::Invalid {
                return Some(RequestLine {
                    method,
                    protocol,
                    request_uri,
                    request_uri_base,
                    query_arguments,
                    query_string,
                });
            }
        } else if parts.len() == 1 {

            // Add support a request line containing only the path name is accepted by servers to maintain compatibility with  clients before the /1.0 specification
            let method = RequestMethod::Get;
            let request_uri = parts.get(0)?.to_string();
            let protocol = RequestProtocol::ZeroDotNine;

            let request_uri_copy = request_uri.clone();
            let mut request_uri_base = request_uri.clone();
            let mut query_string = String::new();
            let mut query_arguments: HashMap<String, String> = HashMap::new();
            
            let uri_parts: Vec<&str> = request_uri_copy.splitn(2, "?").collect();
            if uri_parts.len() == 2 {
                request_uri_base = uri_parts.get(0)?.to_string();
                query_string = uri_parts.get(1)?.to_string();
                if let Some(query_args) = RequestMessage::get_query_args_from_string(&query_string) {
                    query_arguments = query_args;
                }
            }

            return Some(RequestLine {
                method,
                protocol,
                request_uri,
                request_uri_base,
                query_arguments,
                query_string,
            });
        }
        None
    }

    pub fn from_tcp_stream(request: &[u8]) -> Option<RequestMessage> {
        if let Ok(request) = str::from_utf8(request) {
            if request.is_ascii() {
                let mut headers: HashMap<String, String> = HashMap::new();
                let mut body: HashMap<String, String> = HashMap::new();
                let mut request_line: RequestLine = RequestLine {
                    method: RequestMethod::Invalid,
                    protocol: RequestProtocol::Invalid,
                    request_uri: String::new(),
                    request_uri_base: String::new(),
                    query_arguments: HashMap::new(),
                    query_string: String::new(),
                };
                let mut section = RequestSection::RequestLine;
                for mut line in request.lines() {
                    match section {
                        RequestSection::RequestLine => {
                            request_line = RequestMessage::get_request_line(line)?;
                            section = RequestSection::HeaderFields;
                        }
                        RequestSection::HeaderFields => {
                            if line.trim().is_empty() {
                                section = RequestSection::MessageBody;
                            } else {
                                let (header_key, header_value) =
                                    RequestMessage::get_header_field(line)?;
                                headers.insert(header_key, header_value);
                            }
                        }
                        RequestSection::MessageBody => {
                            if line.is_empty() {
                                break;
                            } else if RequestMessage::method_has_request_body(
                                &request_line.method,
                            ) != SettingValence::No
                            {
                                if let Some(body_args) = RequestMessage::get_message_body(line)
                                {
                                    body = body_args;
                                }
                            }
                        }
                    }
                }
                if request_line.method != RequestMethod::Invalid
                    && request_line.protocol != RequestProtocol::Invalid
                {
                    return Some(RequestMessage {
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
        let response = RequestMessage::get_message_body("random=abc&hej=def&def");
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

        let response = RequestMessage::get_message_body("");
        assert!(response.is_none());
    }

    #[test]
    fn test_get_header_field() {
        let response = RequestMessage::get_header_field(
            "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0\r\n",
        );
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(key, "User-Agent".to_string());
        assert_eq!(
            value,
            "Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0".to_string()
        );

        let response = RequestMessage::get_header_field("Cache-Control: no-cache \r\n");
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(key, "Cache-Control".to_string());
        assert_eq!(value, "no-cache".to_string());

        let response = RequestMessage::get_header_field("Just various text here\r\n");
        assert!(response.is_none());

        let response = RequestMessage::get_header_field("");
        assert!(response.is_none());
    }

    #[test]
    fn test_get_request_line() {
        let response = RequestMessage::get_request_line("POST /random?abc=test /0.9\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, RequestMethod::Post);
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
        assert_eq!(response_unpacked.protocol, RequestProtocol::ZeroDotNine);

        let response = RequestMessage::get_request_line("GET / /1.0\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, RequestMethod::Get);
        assert_eq!(response_unpacked.request_uri, String::from("/"));
        assert_eq!(response_unpacked.request_uri_base, String::from("/"));
        assert_eq!(response_unpacked.query_string, String::from(""));
        assert_eq!(response_unpacked.protocol, RequestProtocol::OneDotZero);

        let response =
            RequestMessage::get_request_line("HEAD /moradish.html?test&abc=def /1.1\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, RequestMethod::Head);
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
        assert_eq!(response_unpacked.protocol, RequestProtocol::OneDotOne);

        let response =
            RequestMessage::get_request_line("OPTIONS /random/random2.txt /2.0\r\n");
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(response_unpacked.method, RequestMethod::Options);
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/random/random2.txt")
        );
        assert_eq!(response_unpacked.protocol, RequestProtocol::TwoDotZero);

        let response = RequestMessage::get_request_line("GET / /2.2\r\n");
        assert!(response.is_none());
    }

    #[test]
    fn from_tcp_stream() {
        // GET request with no headers or body
        let response = RequestMessage::from_tcp_stream(b"GET / /2.0\r\n");
        assert!(response.is_some());        
        let response_unwrapped = response.unwrap();
        assert_eq!(
            response_unwrapped.request_line.method,
            RequestMethod::Get
        );
        assert_eq!(response_unwrapped.request_line.request_uri, "/".to_string());
        assert_eq!(
            response_unwrapped.request_line.protocol,
            RequestProtocol::TwoDotZero
        );

        // POST request with random header
        let response = RequestMessage::from_tcp_stream(
            b"POST / /1.0\r\nAgent: Random browser\r\n\r\ntest=abc",
        );
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(
            response_unwrapped.request_line.method,
            RequestMethod::Post
        );
        assert_eq!(
            response_unwrapped.request_line.protocol,
            RequestProtocol::OneDotZero
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
        let response = RequestMessage::from_tcp_stream(b"RANDOM /stuff /2.5\r\n");
        assert!(response.is_none());
        let response = RequestMessage::from_tcp_stream(b"");
        assert!(response.is_none());

        // Get requests should get their message body parsed
        let response = RequestMessage::from_tcp_stream(b"GET / /2.0\r\n\r\nabc=123");
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
        let response = RequestMessage::from_tcp_stream(b"HEAD / /2.0\r\n\r\nabc=123");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert!(response_unwrapped.body.get(&"abc".to_string()).is_none());

        let response = RequestMessage::from_tcp_stream(b"html/index.html\r\n");
        assert!(response.is_some());
        let response_unwrapped = response.unwrap();
        assert_eq!(
            response_unwrapped.request_line.method,
            RequestMethod::Get
        );
        assert_eq!(
            response_unwrapped.request_line.request_uri,
            "html/index.html".to_string()
        );
        assert_eq!(
            response_unwrapped.request_line.protocol,
            RequestProtocol::ZeroDotNine
        );
    }

}
