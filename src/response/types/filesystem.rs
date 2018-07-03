use std;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use response::Type;

pub struct Settings {
    pub root: String,
}

pub struct Responder {
    settings: Settings,
}

// TODO: Must add settings to this
impl Type<Responder> for Responder {
    fn new(settings: HashMap<String, String>) -> Responder {
        // TODO Convert HashMap to config structure here
        Responder {
            settings: Settings {
                root: "./html/".to_string(),
            },
        }
    }

    fn matches(&self, request: &[u8]) -> bool {
        let get = b"GET / ";
        let sleep = b"GET /sleep ";
        let filename = "index.html";
        let path = format!("{}{}", &self.settings.root, filename);
        // TODO Make path dynamic

        return Path::new(&path).exists();
    }

    // Make this respond headers as a HashMap and a string for body
    fn respond(&self, request: &[u8]) -> String {
        let get = b"GET / ";
        let sleep = b"GET /sleep ";

        // TODO Make these more dynamic
        let (status_line, filename) = if request.starts_with(get) {
            ("200 OK", "index.htm")
        } else if request.starts_with(sleep) {
            std::thread::sleep(Duration::from_secs(10));
            ("200 OK", "index.htm")
        } else {
            ("404 NOT FOUND", "404.htm")
        };

        // Read file
        let filename = format!("html/{}", filename);

        // TODO Handle this unwrap
        // TODO Make the path more dynamic
        let mut file = File::open(filename).unwrap();

        // Build response body
        let mut response_body = String::new();

        // TODO Handle this unwrap
        file.read_to_string(&mut response_body).unwrap();

        // TODO Move this to a HTTP response module

        // TODO Make these more dynamic
        // Build HTTP response headers
        let mut response_headers = String::new();
        response_headers.push_str(&format!("HTTP/1.1 {}\r\n", status_line));
        response_headers.push_str("Content-Type: text/html\r\n");

        // TODO Add more headers here
        response_headers.push_str("\r\n");

        // Build HTTP response
        format!("{}{}", response_headers, response_body)
    }
}

#[cfg(test)]
mod filesystem_test {
    use super::*;
    #[test]
    fn matches() {
        let settings = HashMap::new();
        let responder = Responder::new(settings);
        assert!(responder.matches(b"GET / "));
        assert!(responder.matches(b"GET /sleep "));
        assert!(!responder.matches(b"GET /test "));
    }

    #[test]
    fn respond() {
        let settings = HashMap::new();
        let responder = Responder::new(settings);

        let mut file = File::open("html/index.htm").unwrap();

        // Build response body
        let mut response_body = String::new();

        // TODO Handle this unwrap
        file.read_to_string(&mut response_body).unwrap();

        assert_eq!(response_body, responder.respond(b"GET / "));
    }
}
