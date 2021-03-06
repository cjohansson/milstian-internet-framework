//! # TCP HTTP Filesystem Response
//! Used for displaying static resources from the server.
extern crate chrono;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};
use std::net::SocketAddr;

use application_layer::http::request;
use application_layer::http::response;

use mime;
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

    pub fn get_metadata_modified_as_rfc7231(modified: SystemTime) -> String {
        let datetime: DateTime<Utc> = modified.into();
        format!("{}", datetime.format("%a, %d %b %Y %H:%M:%S GMT"))
    }

    pub fn get_rfc7231_as_systemtime(modified: &String) -> Result<SystemTime, String> {
        let offset = Utc::now();
        match offset
            .offset()
            .datetime_from_str(&modified, "%a, %d %b %Y %H:%M:%S GMT")
        {
            Ok(datetime) => {
                datetime.with_timezone(&Utc);
                let modified: SystemTime = datetime.into();
                return Ok(modified);
            }
            Err(error) => {
                return Err(format!(
                    "Failed to parse '{}', error: {:?}",
                    &modified, error
                ));
            }
        }
    }

    pub fn get_modified_hash(data: &SystemTime) -> String {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn get_cache_control(_application: &Application) -> String {
        return "max-age=2592000".to_string(); // TODO Make this dynamic?
    }

    pub fn get_matching_filename(
        request_message: &request::Message,
        application: &Application,
    ) -> Option<String> {
        let mut filename = request_message.request_line.request_uri_base.clone();
        filename = format!("{}{}", application.get_config().filesystem_root, &filename);
        let temp_filename = PathBuf::from(&filename);
        let mut is_dir = false;
        match fs::canonicalize(&temp_filename) {
            Ok(canonical_filename) => {
                match canonical_filename.to_str() {
                    Some(canonical_filename_string) => {
                        let mut filename = canonical_filename_string.to_string();

                        // Is the file inside file-system root?
                        if filename.starts_with(&application.get_config().filesystem_root) {
                            let filename_copy = filename.clone();
                            let splits: Vec<&str> = filename_copy.rsplitn(2, '/').collect();
                            if let Some(basename) = splits.get(0) {
                                // Does base-name not start with dot?
                                if !basename.starts_with(&".".to_string()) {
                                    let mut exists = Path::new(&filename).exists();
                                    if exists {
                                        is_dir = Path::new(&filename).is_dir();
                                        if is_dir {
                                            filename = format!(
                                                "{}/{}",
                                                &filename,
                                                application.get_config().filesystem_directory_index
                                            );
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

                                    if exists && !is_dir {
                                        return Some(filename);
                                    }
                                } else {
                                    eprintln!("Filename {} starts with a dot!", &filename);
                                }
                            } else {
                                eprintln!("Failed to find file base-name {}", &filename);
                            }
                        } else {
                            eprintln!(
                                "File {} is outside of file-system root {}",
                                &filename,
                                application.get_config().filesystem_root
                            );
                        }
                    }
                    None => {
                        eprintln!(
                            "Failed to get canonical path string from {:?}",
                            &canonical_filename
                        );
                    }
                }
            }
            Err(error) => {
                eprintln!(
                    "Failed to get canonical path to {:?}, error: {}, request: {:?}",
                    &temp_filename, error, &request_message
                );
            }
        }
        return None;
    }

    // Make this respond headers as a HashMap and a string for body
    pub fn get_response(
        filename: &String,
        request_message: &request::Message,
        application: &Application,
    ) -> Result<response::Message, String> {
        let mut response_body = Vec::new();

        // Try to open the file
        let file = File::open(filename);
        match file {
            Ok(mut file) => {
                // Try to read the file
                match file.read_to_end(&mut response_body) {
                    Ok(_) => {
                        let mut status_code = "200 OK";

                        let protocol = request::Message::get_protocol_text(
                            &request_message.request_line.protocol,
                        );
                        let mut headers: HashMap<String, String> = HashMap::new();

                        headers.insert("Content-Type".to_string(), mime::from_filename(&filename));

                        if let Ok(metadata) = fs::metadata(&filename) {
                            headers
                                .insert("Content-Length".to_string(), metadata.len().to_string());

                            if let Ok(last_modified) = metadata.modified() {
                                headers.insert(
                                    "Last-Modified".to_string(),
                                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                                );
                                let etag = Responder::get_modified_hash(&last_modified);
                                headers.insert("ETag".to_string(), etag.clone());

                                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                                headers.insert(
                                    "Expires".to_string(),
                                    Responder::get_metadata_modified_as_rfc7231(
                                        last_modified + duration,
                                    ),
                                );

                                if let Some(if_none_match) =
                                    request_message.headers.get("If-None-Match")
                                {
                                    if if_none_match.to_string() == etag {
                                        status_code = "304 Not Modified";
                                        response_body = Vec::new();
                                    }
                                }

                                if status_code != "304 Not Modified" {
                                    if let Some(if_modified_since) =
                                        request_message.headers.get("If-Modified-Since")
                                    {
                                        if let Ok(if_modified_since_systemtime) =
                                            Responder::get_rfc7231_as_systemtime(
                                                &if_modified_since.to_string(),
                                            ) {
                                            if let Ok(duration) = last_modified
                                                .duration_since(if_modified_since_systemtime)
                                            {
                                                /* println!(
                                                    "{:?} vs {:?} = {}",
                                                    &if_modified_since_systemtime,
                                                    &last_modified,
                                                    duration.as_secs()
                                                );  */
                                                if duration.as_secs() <= 0 {
                                                    status_code = "304 Not Modified";
                                                    response_body = Vec::new();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        headers.insert(
                            "Cache-Control".to_string(),
                            Responder::get_cache_control(&application),
                        );

                        // Build HTTP response
                        return Ok(response::Message::new(
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
}

impl ResponderInterface for Responder {
    fn matches(
        &mut self,
        request_message: &request::Message,
        application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
    ) -> bool {
        if let Some(filename) = Responder::get_matching_filename(&request_message, &application) {
            self.filename = Some(filename);
            return true;
        }
        return false;
    }

    // Make this respond headers as a HashMap and a string for body
    fn respond(
        &self,
        request_message: &request::Message,
        application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
    ) -> Result<response::Message, String> {
        // Does filename exist?
        if let Some(filename) = &self.filename {
            let mut response = Responder::get_response(&filename, &request_message, &application)?;
            // Build HTTP response
            return Ok(response);
        } else {
            return Err("Error: Filename missing".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
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
            &request::Message::from_tcp_stream(b"GET / HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
        assert!(responder.matches(
            &request::Message::from_tcp_stream(b"GET /index.htm HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));

        // POST request with random header and null bytes
        let mut request: Vec<u8> =
            b"POST / HTTP/1.0\r\nAgent: Random browser\r\n\r\ntest=abc".to_vec();
        request.push(0);
        request.push(0);
        assert!(responder.matches(
            &request::Message::from_tcp_stream(&request).unwrap(),
            &application,
            &socket,
            &0
        ));

        assert!(!responder.matches(
            &request::Message::from_tcp_stream(b"GET /../README.md HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
        assert!(!responder.matches(
            &request::Message::from_tcp_stream(b"GET /.DS_Store HTTP/1.0").unwrap(),
            &application,
            &socket,
            &0
        ));
        assert!(!responder.matches(
            &request::Message::from_tcp_stream(b"GET /test.htm HTTP/1.1").unwrap(),
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
        let mut responder = Responder::new();
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let filename = "html/index.htm";

        let mut file = File::open(&filename).unwrap();

        // Build response body
        let mut response_body = String::new();

        file.read_to_string(&mut response_body).unwrap();

        let request = request::Message::from_tcp_stream(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        let matches = responder.matches(&request, &application, &socket, &0);
        assert!(matches);

        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified + duration),
                );
            }
            headers.insert("Content-Length".to_string(), metadata.len().to_string());
        }
        headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
        headers.insert(
            "Cache-Control".to_string(),
            Responder::get_cache_control(&application),
        );

        let expected_response = response::Message::new(
            "HTTP/1.1".to_string(),
            "200 OK".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder
            .respond(&request, &application, &socket, &0)
            .unwrap()
            .to_bytes();
        assert_eq!(expected_response, given_response);

        // Matching If Modified Since
        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert("Content-Length".to_string(), metadata.len().to_string());

                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified + duration),
                );

                headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
                headers.insert(
                    "Cache-Control".to_string(),
                    Responder::get_cache_control(&application),
                );

                let response_body_empty = Vec::new();

                let expected_response = response::Message::new(
                    "HTTP/1.1".to_string(),
                    "304 Not Modified".to_string(),
                    headers,
                    response_body_empty,
                ).to_bytes();

                let request_string = format!(
                    "GET /index.htm HTTP/1.1\r\nIf-Modified-Since: {}\r\n\r\n",
                    Responder::get_metadata_modified_as_rfc7231(last_modified)
                );
                let request = request::Message::from_tcp_stream(request_string.as_bytes()).unwrap();

                let given_response = responder
                    .respond(&request, &application, &socket, &0)
                    .unwrap()
                    .to_bytes();
                /* println!(
                    "request: {}, response: {:?}",
                    request_string,
                    str::from_utf8(&given_response)
                ); */
                assert_eq!(expected_response, given_response);
            }
        }

        // Not Matching If Modified Since
        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert("Content-Length".to_string(), metadata.len().to_string());

                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified + duration),
                );
                headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
                headers.insert(
                    "Cache-Control".to_string(),
                    Responder::get_cache_control(&application),
                );

                // Build response body
                let mut response_body = String::new();
                let mut file = File::open(&filename).unwrap();
                file.read_to_string(&mut response_body).unwrap();

                let expected_response = response::Message::new(
                    "HTTP/1.1".to_string(),
                    "200 OK".to_string(),
                    headers,
                    response_body.into_bytes(),
                ).to_bytes();

                let duration = Duration::new(250000, 0);
                let request_string = format!(
                    "GET /index.htm HTTP/1.1\r\nIf-Modified-Since: {}\r\n\r\n",
                    Responder::get_metadata_modified_as_rfc7231(last_modified - duration)
                );
                let request = request::Message::from_tcp_stream(request_string.as_bytes()).unwrap();
                let given_response = responder
                    .respond(&request, &application, &socket, &0)
                    .unwrap()
                    .to_bytes();

                /* println!(
                    "request: {}, response: {:?}, expected response: {:?}",
                    request_string,
                    str::from_utf8(&given_response),
                    str::from_utf8(&expected_response)
                ); */
                assert_eq!(expected_response, given_response);
            }
        }

        // If None Match
        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert("Content-Length".to_string(), metadata.len().to_string());

                let request_string = format!(
                    "GET /index.htm HTTP/1.1\r\nIf-None-Match: {}\r\n\r\n",
                    Responder::get_modified_hash(&last_modified)
                );
                let request = request::Message::from_tcp_stream(request_string.as_bytes()).unwrap();
                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified + duration),
                );

                headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
                headers.insert(
                    "Cache-Control".to_string(),
                    Responder::get_cache_control(&application),
                );

                let response_body = Vec::new();

                let expected_response = response::Message::new(
                    "HTTP/1.1".to_string(),
                    "304 Not Modified".to_string(),
                    headers,
                    response_body,
                ).to_bytes();

                let given_response = responder
                    .respond(&request, &application, &socket, &0)
                    .unwrap()
                    .to_bytes();
                assert_eq!(expected_response, given_response);
            }
        }

        // Not Matching If None Match
        let mut headers: HashMap<String, String> = HashMap::new();
        if let Ok(metadata) = fs::metadata(&filename) {
            if let Ok(last_modified) = metadata.modified() {
                headers.insert("Content-Length".to_string(), metadata.len().to_string());

                headers.insert(
                    "Last-Modified".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified),
                );
                headers.insert(
                    "ETag".to_string(),
                    Responder::get_modified_hash(&last_modified),
                );
                let duration = Duration::new(2592000, 0); // TODO Make this dynamic
                headers.insert(
                    "Expires".to_string(),
                    Responder::get_metadata_modified_as_rfc7231(last_modified + duration),
                );
                headers.insert("Content-Type".to_string(), mime::from_filename(&filename));
                headers.insert(
                    "Cache-Control".to_string(),
                    Responder::get_cache_control(&application),
                );

                // Build response body
                let mut response_body = String::new();
                let mut file = File::open(&filename).unwrap();
                file.read_to_string(&mut response_body).unwrap();

                let expected_response = response::Message::new(
                    "HTTP/1.1".to_string(),
                    "200 OK".to_string(),
                    headers,
                    response_body.into_bytes(),
                ).to_bytes();

                let duration = Duration::new(250000, 0);
                let last_modified = last_modified - duration;
                let request_string = format!(
                    "GET /index.htm HTTP/1.1\r\nIf-None-Match: {}\r\n\r\n",
                    Responder::get_modified_hash(&last_modified)
                );
                let request = request::Message::from_tcp_stream(request_string.as_bytes()).unwrap();
                let given_response = responder
                    .respond(&request, &application, &socket, &0)
                    .unwrap()
                    .to_bytes();

                /* println!(
                    "request: {}, response: {:?}, expected response: {:?}",
                    request_string,
                    str::from_utf8(&given_response),
                    str::from_utf8(&expected_response)
                ); */
                assert_eq!(expected_response, given_response);
            }
        }
    }
}
