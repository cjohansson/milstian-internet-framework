mod types;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

use self::types::filesystem;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, settings: &HashMap<String, String>) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let mut response = String::from("");

        // TODO Make response types dynamic here
        let responder = filesystem::Responder::new(&settings);

        if responder.matches(&buffer) {
            response = responder.respond(&buffer);
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
trait Type<'a, T> {
    fn new(settings: &'a HashMap<String, String>) -> T;
    fn matches(&self, request: &[u8]) -> bool;
    fn respond(&self, request: &[u8]) -> String;
}
