//! # TCP HTTP File not found Response
//! Used for displaying that a resource was not found on the server.

use std::net::SocketAddr;
use std::path::Path;

use application_layer::http::request;
use application_layer::http::response;

use response::tcp::http::filesystem;
use response::tcp::http::ResponderInterface;
use Application;

#[derive(Clone)]
pub struct Responder {
    pub filename: Option<String>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder { filename: None }
    }
}

impl ResponderInterface for Responder {
    fn matches(
        &mut self,
        _request_message: &request::Message,
        application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u8,
    ) -> bool {
        let filename = format!(
            "{}/{}",
            application.get_config().filesystem_root,
            application.get_config().file_not_found_file
        );
        let exists = Path::new(&filename).exists();
        let mut is_dir = false;
        if exists {
            is_dir = Path::new(&filename).is_dir();
            if is_dir {
                eprintln!("File not found because file is a directory {}", &filename);
            }
        } else {
            eprintln!("File not found file does not exists {}", &filename);
        }
        self.filename = Some(filename);
        return exists && !is_dir;
    }

    fn respond(
        &self,
        request_message: &request::Message,
        application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u8,
    ) -> Result<response::Message, String> {
        if let Some(filename) = &self.filename {
            let mut response =
                filesystem::Responder::get_response(filename, &request_message, &application)?;
            response.status = "404 File Not Found".to_string();
            return Ok(response);
        } else {
            return Err("Error: File Not Found Filename missing".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Duration;

    use application_layer::http::response;
    use mime;

    use Config;

    #[test]
    fn matches() {
        let config = Config {
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let application = Application::new(config);
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut responder = Responder::new();
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index3.htm HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index.htm HTTP/1.1").unwrap(),
            &application,
            &socket,
            &0
        ));

        let config = Config {
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404_file.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let application = Application::new(config);
        let mut responder = Responder::new();
        assert!(!responder.matches(
            &request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
    }

    #[test]
    fn respond() {
        let config = Config {
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let application = Application::new(config);
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut responder = Responder::new();

        let filename = "html/404.htm";

        let mut file = File::open(&filename).unwrap();

        // Build response body
        let mut response_body = String::new();

        file.read_to_string(&mut response_body).unwrap();

        let request =
            request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.1\r\n\r\n").unwrap();

        let matches = responder.matches(&request, &application, &socket, &0);
        assert!(matches);

        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert(
                    "Last-Modified".to_string(),
                    filesystem::Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    filesystem::Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    filesystem::Responder::get_metadata_modified_as_rfc7231(
                        last_modified + duration,
                    ),
                );
            }
            headers.insert("Content-Length".to_string(), metadata.len().to_string());
        }
        headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
        headers.insert(
            "Cache-Control".to_string(),
            filesystem::Responder::get_cache_control(&application),
        );

        let expected_response = response::Message::new(
            "HTTP/1.1".to_string(),
            "404 File Not Found".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder
            .respond(&request, &application, &socket, &0)
            .unwrap()
            .to_bytes();
        assert_eq!(expected_response, given_response);
    }
}
