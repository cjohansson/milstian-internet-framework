mod types;

use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use self::types::http::filesystem;
use Config;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, config: Config) {
        // Create a array with 512 elements containing the value 0
        let mut buffer = [0; 512];

        if let Ok(_) = stream.read(&mut buffer) {
            let mut response = String::from("");
            let mut filesystem = filesystem::Responder::new();

            if filesystem.matches(&buffer, &config) {
                if let Ok(filesystem_response) = filesystem.respond(&buffer, &config) {
                    response = filesystem_response;
                }
            }
            // TODO Add more http response types here: not found, page, ajax, invalid request

            if !response.is_empty() {
                // Flush HTTP response
                match stream.write(response.as_bytes()) {
                    Ok(_) => {
                        if let Err(error) = stream.flush() {
                            eprintln!(
                                "Failed to flush TCP stream, error: {}",
                                error
                            );
                        }
                    },
                    Err(error) => {
                        eprintln!(
                            "Failed to write TCP stream, error: {}",
                            error
                        );
                    }
                }
            } else {
                eprintln!(
                    "Found no response for request {:?}",
                    str::from_utf8(&buffer)
                );
            }
        }
    }
}

// This is the trait that all response types implement
trait Type<T> {
    fn matches(&mut self, request: &[u8], config: &Config) -> bool;
    fn respond(&self, request: &[u8], config: &Config) -> Result<String, String>;
}
