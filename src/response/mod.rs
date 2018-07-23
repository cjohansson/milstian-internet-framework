mod types;

use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use self::types::filesystem;
use Config;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, config: Config) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        // TODO Handle this unwrap
        stream.read(&mut buffer).unwrap();

        let mut response = String::from("");
        let mut filesystem = filesystem::Responder::new();

        if filesystem.matches(&buffer, &config) {
            response = filesystem.respond(&buffer, &config);
        }
        // TODO Add more response types here: page, ajax

        if !response.is_empty() {
            // Flush HTTP response
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            eprintln!(
                "Found no response for request {:?}",
                str::from_utf8(&buffer)
            );
        }
    }
}

// This is the trait that all response types implement
trait Type<T> {
    fn matches(&mut self, request: &[u8], config: &Config) -> bool;
    fn respond(&self, request: &[u8], config: &Config) -> String;
}
