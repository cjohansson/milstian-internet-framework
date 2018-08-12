use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use application_layer_protocol::http;
use mime;
use response::types::http::filesystem;
use response::Type;
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
            let filename = format!("{}{}", &config.filesystem_root, &config.file_not_found_file);
            let exists = Path::new(&filename).exists();
            let mut is_dir = false;
            if exists {
                is_dir = Path::new(&filename).is_dir();
            }
            self.filename = Some(filename);
            self.request_message = Some(request_message);

            return exists && !is_dir;
        } else {
            eprintln!("Failed to get HTTP request from TCP stream");
        }
        false
    }

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
                            let mut status_code = "404 File Not Found";

                            let protocol = http::request::Message::get_protocol_text(
                                &self.request_message.as_ref().unwrap().request_line.protocol,
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
                                        filesystem::Responder::get_metadata_modified_as_rfc7231(
                                            last_modified,
                                        ),
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
