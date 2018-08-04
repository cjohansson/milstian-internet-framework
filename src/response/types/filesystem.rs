use std;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use mime;
use response::Type;
use transport_protocol::http;
use Config;

pub struct Responder {
    pub filename: Option<String>,
    pub request_message: Option<http::request::Message>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder {
            filename: None,
            request_message: None,
        }
    }
}

impl Type<Responder> for Responder {
    fn matches(&mut self, request: &[u8], config: &Config) -> bool {
        if let Some(request_message) = http::request::Message::from_tcp_stream(request) {
            let mut filename = request_message.request_line.request_uri_base.clone();
            if filename.starts_with("/") {
                filename.remove(0);
            }
            let mut filename = format!("{}{}", &config.filesystem_root, &filename);
            let mut exists = Path::new(&filename).exists();
            let mut is_dir = false;
            if exists {
                is_dir = Path::new(&filename).is_dir();
                if is_dir {
                    filename = format!("{}{}", &filename, &config.filesystem_index);
                    exists = Path::new(&filename).exists();
                    is_dir = Path::new(&filename).is_dir()
                }
            }
            self.request_message = Some(request_message);
            self.filename = Some(filename);
            return exists && !is_dir;
        }
        false
    }

    // Make this respond headers as a HashMap and a string for body
    fn respond(&self, _request: &[u8], _config: &Config) -> Result<String, String> {
        let mut response_body = String::new();

        // Does filename exist?
        if let Some(filename) = &self.filename {
            // Try to open the file
            let file = File::open(filename);
            match file {
                Ok(mut file) => {
                    // Try to read the file
                    match file.read_to_string(&mut response_body) {
                        Ok(_) => {
                            // TODO Make this dynamic
                            let status_code = "200 OK";

                            let protocol = http::request::Message::get_protocol_text(
                                &self.request_message.as_ref().unwrap().request_line.protocol,
                            );
                            let mut headers: HashMap<String, String> = HashMap::new();

                            headers
                                .insert("Content-Type".to_string(), mime::from_filename(&filename));

                            // Build HTTP response
                            return Ok(http::response::Message::new(
                                protocol.to_string(),
                                status_code.to_string(),
                                headers,
                                response_body,
                            ).to_string());
                        }
                        Err(e) => {
                            return Err(format!(
                                "Error: Failed to read file {}, error: {:?}",
                                filename, e
                            ));
                        }
                    }
                }
                Err(e) => {
                    return Err(format!(
                        "Error: Failed to open file {}, error: {:?}",
                        filename, e
                    ));
                }
            }
        } else {
            return Err("Error: Filename missing".to_string());
        }
    }
}

#[cfg(test)]
mod filesystem_test {
    use super::*;
    #[test]
    fn matches() {
        let config = Config {
            filesystem_index: "index.htm".to_string(),
            filesystem_root: "./html/".to_string(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };
        let mut responder = Responder::new();
        assert!(responder.matches(b"GET / HTTP/1.0", &config));
        assert!(responder.matches(b"GET /index.htm HTTP/1.0", &config));
        assert!(!responder.matches(b"GET /test.htm HTTP/1.1", &config));
    }

    #[test]
    fn respond() {
        let config = Config {
            filesystem_index: "index.htm".to_string(),
            filesystem_root: "./html/".to_string(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };
        let mut responder = Responder::new();

        let mut file = File::open("html/index.htm").unwrap();

        // Build response body
        let mut response_body = String::new();

        file.read_to_string(&mut response_body).unwrap();

        let request = b"GET / HTTP/1.1";

        let matches = responder.matches(request, &config);
        assert!(matches);

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        let expected_response = http::response::Message::new(
            "HTTP/1.1".to_string(),
            "200 OK".to_string(),
            headers,
            response_body,
        ).to_string();

        let given_response = responder.respond(request, &config).unwrap();
        assert_eq!(expected_response, given_response);
    }
}
