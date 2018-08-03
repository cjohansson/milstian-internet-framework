use std;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use transport_protocol::http;
use response::Type;
use Config;

pub struct Responder {
    pub filename: Option<String>,
    pub request_message: Option<http::RequestMessage>,
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
        if let Some(request_message) = http::RequestMessage::from_tcp_stream(request) {
            let mut filename = request_message.request_line.request_uri_base.clone();
            if filename.starts_with("/") {
                filename.remove(0);
                println!("New filename is: {}", &filename);
            }
            println!("Filename is: {}", &filename);
            let mut filename = format!("{}{}", &config.filesystem_root, &filename);
            let mut exists = Path::new(&filename).exists();
            let mut is_dir = false;
            if exists {
                is_dir = Path::new(&filename).is_dir();
                if is_dir {
                    filename = format!("{}{}", &filename, &config.filesystem_index);
                    println!("Composite filename: {}", &filename);
                    exists = Path::new(&filename).exists();
                    is_dir = Path::new(&filename).is_dir()
                }
            }
            println!("Final filename: {}", &filename);
            self.request_message = Some(request_message);
            self.filename = Some(filename);
            return exists && !is_dir;
        }
        false
    }

    // Make this respond headers as a HashMap and a string for body
    fn respond(&self, _request: &[u8], _config: &Config) -> Result<String, String> {
        let mut response_body = String::new();
        let mut response_headers = String::new();

        // Does filename exist?
        if let Some(filename) = &self.filename {

            // Try to open the file
            let file = File::open(filename);
            match file {
                Ok(mut file) =>{

                    // Try to read the file
                    match file.read_to_string(&mut response_body) {
                        Ok(_) => {

                            // TODO Make this dynamic
                            let status_code = "200 OK";

                            // let procotol = self.request_message.unwrap().request_line.protocol
                            let protocol = http::RequestMessage::get_protocol_text(&self.request_message.as_ref().unwrap().request_line.protocol);

                            // TODO Make these more dynamic
                            // Build HTTP response headers
                            response_headers.push_str(&format!("{} {}\r\n", protocol, status_code));
                            response_headers.push_str("Content-Type: text/html\r\n");

                            // TODO Add more headers here
                            response_headers.push_str("\r\n");

                            // Build HTTP response
                            return Ok(format!("{}{}", response_headers, response_body));
                        },
                        Err(e) => {
                            return Err(format!("Error: Failed to read file {}, error: {:?}", filename, e));
                        }
                    }
                },
                Err(e) => {
                    return Err(format!("Error: Failed to open file {}, error: {:?}", filename, e));
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

        // Add HTTP headers
        response_body = format!(
            "{}{}",
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n", response_body
        );

        let response = responder.respond(request, &config).unwrap();
        assert_eq!(response_body, response);
    }
}
