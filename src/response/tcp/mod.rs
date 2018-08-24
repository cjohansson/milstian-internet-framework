pub mod http;

use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::str;

use Config;

// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and finds appropriate response handler
    pub fn dispatch_request(mut stream: TcpStream, socket: SocketAddr, config: Config) {
        // Create a array with 512 elements containing the value 0
        let mut temp_buffer = [0; 512];
        let mut buffer: Vec<u8> = Vec::new();

        println!("Received stream from {:?}", socket);

        if let Ok(read_size) = stream.read(&mut temp_buffer) {
            // Move all non-empty values to new buffer
            for value in temp_buffer.iter() {
                if value != &0 {
                    buffer.push(*value);
                }
                if buffer.len() > config.tcp_limit {
                    break;
                }
            }

            // Did we read maximum number of bytes?
            if read_size == 512 && config.tcp_limit > 512 {
                loop {
                    match stream.read(&mut temp_buffer) {
                        Ok(read_size) => {
                            // Move all non-empty values to new buffer
                            for value in temp_buffer.iter() {
                                if value != &0 {
                                    buffer.push(*value);
                                }
                            }

                            if read_size < 512 || buffer.len() < config.tcp_limit {
                                break;
                            }
                        }
                        Err(error) => {
                            eprintln!("Failed to read from stream, error: {}", error);
                            break;
                        }
                    }
                }
            }
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
