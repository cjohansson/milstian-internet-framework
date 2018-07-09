mod types;

use std::io::prelude::*;
use std::net::TcpStream;

use self::types::filesystem;
use Config;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, config: Config) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let mut response = String::from("");

        let filesystem = filesystem::Responder {};

        if filesystem.matches(&buffer, &config) {
            response = filesystem.respond(&buffer, &config);
        }
        // TODO Add more response types here

        if !response.is_empty() {
            // Flush HTTP response
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            println!("Found no response for request");
        }
    }
}

// This is the trait that all response types implement
trait Type<T> {
    fn matches(&self, request: &[u8], config: &Config) -> bool;
    fn respond(&self, request: &[u8], config: &Config) -> String;
}
