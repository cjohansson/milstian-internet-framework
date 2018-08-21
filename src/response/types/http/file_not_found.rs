use std::path::Path;

use application_layer::http::request;
use response::types::http::filesystem;
use Config;

pub struct Responder {
    pub filename: Option<String>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder { filename: None }
    }

    pub fn matches(&mut self, _request_message: &request::Message, config: &Config) -> bool {
        let filename = format!(
            "{}/{}",
            &config.filesystem_root, &config.file_not_found_file
        );
        let exists = Path::new(&filename).exists();
        let mut is_dir = false;
        if exists {
            is_dir = Path::new(&filename).is_dir();
            if is_dir {
                eprintln!("File not found file is directory {}", &filename);
            }
        } else {
            eprintln!("File not found file does not exists {}", &filename);
        }
        self.filename = Some(filename);
        return exists && !is_dir;
    }

    pub fn respond(
        &self,
        request_message: &request::Message,
        config: &Config,
    ) -> Result<Vec<u8>, String> {
        if let Some(filename) = &self.filename {
            let mut response =
                filesystem::Responder::get_response(filename, &request_message, &config)?;
            response.status = "404 File Not Found".to_string();
            return Ok(response.to_bytes());
        } else {
            return Err("Error: File Not Found Filename missing".to_string());
        }
    }
}

#[cfg(test)]
mod file_not_found_test {
    use super::*;

    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::time::Duration;

    use application_layer::http::response;
    use mime;

    #[test]
    fn matches() {
        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
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

        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404_file.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let mut responder = Responder::new();
        assert!(!responder.matches(
            &request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.0").unwrap(),
            &config
        ));
    }

    #[test]
    fn respond() {
        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let mut responder = Responder::new();

        let filename = "html/404.htm";

        let mut file = File::open(&filename).unwrap();

        // Build response body
        let mut response_body = String::new();

        file.read_to_string(&mut response_body).unwrap();

        let request =
            request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.1\r\n\r\n").unwrap();

        let matches = responder.matches(&request, &config);
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
            filesystem::Responder::get_cache_control(&config),
        );

        let expected_response = response::Message::new(
            "HTTP/1.1".to_string(),
            "404 File Not Found".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder.respond(&request, &config).unwrap();
        assert_eq!(expected_response, given_response);
    }
}
