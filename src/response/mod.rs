mod types;

use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use self::types::http;
use Config;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, config: Config) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        if let Ok(_) = stream.read(&mut buffer) {
            let mut response = Vec::new();

            let mut http_dispatcher = http::Dispatcher::new();

            if http_dispatcher.matches(&buffer, &config) {
                match http_dispatcher.respond(&buffer, &config) {
                    Ok(http_response) => {
                        response = http_response;
                    }
                    Err(error) => {
                        eprintln!("Got empty HTTP response! Error: {}", error);
                    }
                }
            }
            // TODO Add more application layer protocols here

            if !response.is_empty() {
                // Flush HTTP response
                match stream.write(&response) {
                    Ok(_) => {
                        if let Err(error) = stream.flush() {
                            eprintln!("Failed to flush TCP stream, error: {}", error);
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to write TCP stream, error: {}", error);
                    }
                }
            } else {
                eprintln!(
                    "Found no response for TCP request {:?}",
                    str::from_utf8(&buffer)
                );
            }
        }
    }
}

// This is the trait that all response types implement
trait Type<T> {
    fn matches(&mut self, request: &[u8], config: &Config) -> bool;
    fn respond(&self, request: &[u8], config: &Config) -> Result<Vec<u8>, String>;
}
