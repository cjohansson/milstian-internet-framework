mod types;

use std::io::prelude::*;
use std::net::TcpStream;

use self::types::Filesystem;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let mut response = String::from("");

        if Filesystem::matches(&buffer) {
            response = Filesystem::respond(&buffer);
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
trait Type {
    fn matches(request: &[u8]) -> bool;
    fn respond(request: &[u8]) -> String;
}

// TODO Create response types
