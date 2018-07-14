use std::collections::HashMap;
use std::str;

pub struct HttpRequestMessage {
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
    request_line: HttpRequestLine
}

#[derive(Debug)]
pub struct HttpRequestLine {
    method: HttpRequestMethod,
    protocol: HttpRequestProtocol,
    request_uri: String,
    request_uri_base: String,
    query_string: String,
}

#[derive(Debug, Eq, PartialEq)]
enum HttpRequestMethod {
    Connect,
    Delete,
    Get,
    Head,
    Invalid,
    Options,
    Patch,
    Post,
    Put,
    Trace
}

#[derive(Debug, Eq, PartialEq)]
enum HttpRequestProtocol {
    Invalid,
    TwoDotZero,
    OneDotZero,
    OneDotOne,
    ZeroDotNine,
}

enum HttpRequestSection {
    RequestLine,
    HeaderFields,
    MessageBody
}

impl HttpRequestMessage {

    // This associated function should parse body based on encoding
    // TODO: Implement this
    pub fn get_message_body(encoding: Option<String>, body: &str) -> Option<HashMap<String, String>> {
        None
    }

    pub fn get_header_field(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        if parts.len() == 2 {

            let header_key = parts.get(0)?.trim().to_string();
            let header_value = parts.get(1)?.trim().to_string();
            return Some((header_key, header_value));

        }
        None
    }

    // TODO This associated function should parse GET arguments as well
    pub fn get_request_line(line: &str) -> Option<HttpRequestLine> {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 3 {

            let request_method = match parts.get(0)?.as_ref() {
                "CONNECT" => HttpRequestMethod::Connect,
                "DELETE" => HttpRequestMethod::Delete,
                "GET" => HttpRequestMethod::Get,
                "HEAD" => HttpRequestMethod::Head,
                "OPTIONS" => HttpRequestMethod::Options,
                "PATCH" => HttpRequestMethod::Patch,
                "PUT" => HttpRequestMethod::Put,
                "POST" => HttpRequestMethod::Post,
                "TRACE" => HttpRequestMethod::Trace,
                __ => HttpRequestMethod::Invalid
            };

            let request_uri = parts.get(1)?.to_string();
            let request_uri_copy = request_uri.clone();
            let mut request_uri_base = String::new();
            let mut query_string = String::new();
            let uri_parts: Vec<&str> = request_uri_copy.splitn(2, "?").collect();
            if uri_parts.len() == 2 {
                request_uri_base = uri_parts.get(0)?.to_string();
                query_string = uri_parts.get(1)?.to_string();
            }

            let request_protocol = match parts.get(2)?.as_ref() {
                "HTTP/0.9" => HttpRequestProtocol::ZeroDotNine,
                "HTTP/1.0" => HttpRequestProtocol::OneDotZero,
                "HTTP/1.1" => HttpRequestProtocol::OneDotOne,
                "HTTP/2.0" => HttpRequestProtocol::TwoDotZero,
                _ => HttpRequestProtocol::Invalid
            };

            if request_method != HttpRequestMethod::Invalid
                && request_protocol != HttpRequestProtocol::Invalid
            {
                return Some(HttpRequestLine {
                    method: request_method,
                    protocol: request_protocol,
                    request_uri: request_uri,
                    request_uri_base: request_uri_base,
                    query_string: query_string
                });
            }
        }
        None
    }

    // TODO Implement this
    pub fn from_tcp_stream(request: &[u8]) -> Option<HttpRequestMessage> {
        if let Ok(request) = str::from_utf8(request) {
            if request.is_ascii() {
                let mut headers: HashMap<String, String> = HashMap::new();
                let mut body: HashMap<String, String> = HashMap::new();
                let mut request_line: HttpRequestLine = HttpRequestLine {
                    method: HttpRequestMethod::Invalid,
                    protocol: HttpRequestProtocol::Invalid,
                    request_uri: String::new(),
                    request_uri_base: String::new(),
                    query_string: String::new()
                };
                let mut section = HttpRequestSection::RequestLine;
                for mut line in request.lines() {
                    match section {
                        HttpRequestSection::RequestLine => {
                            request_line = HttpRequestMessage::get_request_line(line)?;
                            section = HttpRequestSection::HeaderFields;
                        },
                        HttpRequestSection::HeaderFields => {
                            if line.trim().is_empty() {
                                section = HttpRequestSection::MessageBody;
                            } else {
                                let (header_key, header_value) = HttpRequestMessage::get_header_field(line)?;
                                headers.insert(header_key, header_value);
                            }
                        },
                        HttpRequestSection::MessageBody => {
                            if line.is_empty() {
                                break; // TODO Verify this
                            } else {
                                // TODO: Do stuff here
                            }
                        }
                    }
                }
                if request_line.method != HttpRequestMethod::Invalid
                    && request_line.protocol != HttpRequestProtocol::Invalid
                {
                    return Some(HttpRequestMessage {
                        body,
                        headers,
                        request_line
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
    fn test_get_header_field() {
        let response = HttpRequestMessage::get_header_field(
            "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0\r\n"
        );
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(
            key,
            "User-Agent".to_string()
        );
        assert_eq!(
            value,
            "Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0".to_string()
        );

        let response = HttpRequestMessage::get_header_field(
            "Cache-Control: no-cache \r\n"
        );
        assert!(response.is_some());

        let (key, value) = response.unwrap();
        assert_eq!(
            key,
            "Cache-Control".to_string()
        );
        assert_eq!(
            value,
            "no-cache".to_string()
        );

        let response = HttpRequestMessage::get_header_field(
            "Just various text here\r\n"
        );
        assert!(response.is_none());

        let response = HttpRequestMessage::get_header_field(
            ""
        );
        assert!(response.is_none());
    }

    #[test]
    fn test_get_request_line() {
        let response = HttpRequestMessage::get_request_line(
            "POST /random HTTP/0.9\r\n"
        );
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(
            response_unpacked.method,
            HttpRequestMethod::Post
        );
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/random")
        );
        assert_eq!(
            response_unpacked.protocol,
            HttpRequestProtocol::ZeroDotNine
        );

        let response = HttpRequestMessage::get_request_line(
            "GET / HTTP/1.0\r\n"
        );
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(
            response_unpacked.method,
            HttpRequestMethod::Get
        );
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/")
        );
        assert_eq!(
            response_unpacked.protocol,
            HttpRequestProtocol::OneDotZero
        );

        let response = HttpRequestMessage::get_request_line(
            "HEAD /moradish.html HTTP/1.1\r\n"
        );
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(
            response_unpacked.method,
            HttpRequestMethod::Head
        );
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/moradish.html")
        );
        assert_eq!(
            response_unpacked.protocol,
            HttpRequestProtocol::OneDotOne
        );


        let response = HttpRequestMessage::get_request_line(
            "OPTIONS /random/random2.txt HTTP/2.0\r\n"
        );
        assert!(response.is_some());

        let response_unpacked = response.unwrap();
        assert_eq!(
            response_unpacked.method,
            HttpRequestMethod::Options
        );
        assert_eq!(
            response_unpacked.request_uri,
            String::from("/random/random2.txt")
        );
        assert_eq!(
            response_unpacked.protocol,
            HttpRequestProtocol::TwoDotZero
        );

        let response = HttpRequestMessage::get_request_line(
            "GET / HTTP/2.2\r\n"
        );
        assert!(response.is_none());
    }

    #[test]
    fn from_tcp_stream() {
        let response = HttpRequestMessage::from_tcp_stream(
            b"GET / HTTP/2.0\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequestMessage::from_tcp_stream(
            b"POST / HTTP/2.0\r\nAgent: Random browser\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequestMessage::from_tcp_stream(
            b"RANDOM /stuff HTTP/2.5\r\n"
        );
        assert!(response.is_none());

        let response = HttpRequestMessage::from_tcp_stream(
            b""
        );
        assert!(response.is_none());
    }

}



