use application_layer::http::request;
use application_layer::http::response;
use std::collections::HashMap;
use Config;

pub struct Responder {
    pub filename: Option<String>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder { filename: None }
    }

    pub fn matches(&mut self, _request_message: &request::Message, _config: &Config) -> bool {
        true
    }

    pub fn respond(
        &self,
        request_message: &request::Message,
        _config: &Config,
    ) -> Result<Vec<u8>, String> {
        let status_code = "500 Internal Server Error";
        let protocol = request::Message::get_protocol_text(&request_message.request_line.protocol);
        let headers: HashMap<String, String> = HashMap::new();
        let response_body = Vec::new();

        // Build HTTP response
        return Ok(response::Message::new(
            protocol.to_string(),
            status_code.to_string(),
            headers,
            response_body,
        ).to_bytes());
    }
}

#[cfg(test)]
mod error_test {
    use super::*;

    use application_layer::http::response;
    use std::collections::HashMap;

    #[test]
    fn matches() {
        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root("./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };
        let mut responder = Responder::new();
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.0").unwrap(),
            &config
        ));
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index3.htm HTTP/1.0").unwrap(),
            &config
        ));
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index.htm HTTP/1.1").unwrap(),
            &config
        ));
    }

    #[test]
    fn respond() {
        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root("./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };
        let mut responder = Responder::new();

        // Build response body
        let mut response_body = String::new();
        let request =
            request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.1\r\n\r\n").unwrap();
        let matches = responder.matches(&request, &config);
        assert!(matches);

        let mut headers: HashMap<String, String> = HashMap::new();

        let expected_response = response::Message::new(
            "HTTP/1.1".to_string(),
            "500 Internal Server Error".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder.respond(&request, &config).unwrap();
        assert_eq!(expected_response, given_response);
    }
}
