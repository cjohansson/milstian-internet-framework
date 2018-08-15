extern crate chrono;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use chrono::offset::Utc;
use chrono::DateTime;

use application_layer_protocol::http;
use application_layer_protocol::http::request::Message;
use mime;
use Config;

pub struct Responder {
    pub filename: Option<String>,
}

impl Responder {
    pub fn get_metadata_modified_as_rfc7231(modified: SystemTime) -> String {
        let datetime: DateTime<Utc> = modified.into();
        format!("{}", datetime.format("%a, %d %b %Y %H:%M:%S GMT"))
    }

    pub fn new() -> Responder {
        Responder {
            filename: None,
        }
    }
    
    pub fn matches(&mut self, request_message: &Message, config: &Config) -> bool {
        let mut filename = request_message.request_line.request_uri_base.clone();
        filename = format!("{}{}", &config.filesystem_root, &filename);
        let temp_filename = PathBuf::from(&filename);
        let mut exists = false;
        let mut is_dir = false;
        match fs::canonicalize(&temp_filename) {
            Ok(canonical_filename) => {
                match canonical_filename.to_str() {
                    Some(canonical_filename_string) => {
                        let mut filename = canonical_filename_string.to_string();
                        // TODO Need to check that the canonical filename is below the canonical root

                        exists = Path::new(&filename).exists();
                        if exists {
                            is_dir = Path::new(&filename).is_dir();
                            if is_dir {
                                filename = format!("{}/{}", &filename, &config.filesystem_directory_index);
                                exists = Path::new(&filename).exists();
                                is_dir = Path::new(&filename).is_dir()
                            }
                        }
                        if !exists {
                            eprintln!("File does not exists {}", &filename);
                        }
                        if is_dir {
                            eprintln!("File is a directory {}", &filename);
                        }

                        self.filename = Some(filename);
                    },
                    None => {
                        eprintln!("Failed to get canonical path string from {:?}", &canonical_filename);
                    }
                }
            },
            Err(error) => {
                eprintln!("Failed to get canonical path to {:?}, error: {}", &temp_filename, error);
            }
        }
        return exists && !is_dir;
    }

    // Make this respond headers as a HashMap and a string for body
    pub fn get_response(filename: &String, request_message: &Message) -> Result<http::response::Message, String> {
        let mut response_body = Vec::new();

        // Try to open the file
        let file = File::open(filename);
        match file {
            Ok(mut file) => {
                // Try to read the file
                match file.read_to_end(&mut response_body) {
                    Ok(_) => {
                        let mut status_code = "200 OK";

                        let protocol = http::request::Message::get_protocol_text(
                            &request_message.request_line.protocol,
                        );
                        let mut headers: HashMap<String, String> = HashMap::new();

                        headers
                            .insert("Content-Type".to_string(), mime::from_filename(&filename));

                        if let Ok(metadata) = fs::metadata(&filename) {
                            headers.insert(
                                "Content-Length".to_string(),
                                metadata.len().to_string(),
                            );

                            if let Ok(last_modified) = metadata.modified() {
                                headers.insert(
                                    "Last-Modified".to_string(),
                                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                                );

                                // TODO Add Expires, Etag and Cache-Control here
                                // TODO Support If-Modified-Since and If-None-Match here
                            }
                        }

                        // Build HTTP response
                        return Ok(http::response::Message::new(
                            protocol.to_string(),
                            status_code.to_string(),
                            headers,
                            response_body,
                        ));
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
    }

    // Make this respond headers as a HashMap and a string for body
    pub fn respond(&self, request_message: &Message) -> Result<Vec<u8>, String> {
        // Does filename exist?
        if let Some(filename) = &self.filename {
            let mut response = Responder::get_response(&filename, &request_message)?;
            // Build HTTP response
            return Ok(response.to_bytes());
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
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: "./html/".to_string(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };

        let mut responder = Responder::new();
        assert!(responder.matches(&http::request::Message::from_tcp_stream(b"GET / HTTP/1.0").unwrap(), &config));
        assert!(responder.matches(&http::request::Message::from_tcp_stream(b"GET /index.htm HTTP/1.0").unwrap(), &config));
        assert!(!responder.matches(&http::request::Message::from_tcp_stream(b"GET /test.htm HTTP/1.1").unwrap(), &config));
    }

    #[test]
    fn respond() {
        let config = Config {
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: "./html/".to_string(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
        };
        let mut responder = Responder::new();

        let filename = "html/index.htm";

        let mut file = File::open(&filename).unwrap();

        // Build response body
        let mut response_body = String::new();

        file.read_to_string(&mut response_body).unwrap();

        let request = http::request::Message::from_tcp_stream(b"GET / HTTP/1.1\r\n\r\n").unwrap();

        let matches = responder.matches(&request, &config);
        assert!(matches);

        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
            }
            headers.insert("Content-Length".to_string(), metadata.len().to_string());
        }
        headers.insert("Content-Type".to_string(), mime::from_filename(&filename));

        let expected_response = http::response::Message::new(
            "HTTP/1.1".to_string(),
            "200 OK".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder.respond(&request).unwrap();
        assert_eq!(expected_response, given_response);
    }
}
