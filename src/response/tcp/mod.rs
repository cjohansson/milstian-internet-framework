//! # Namespace for TCP/IP responses

pub mod http;

use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::str;

use response::tcp::http::ResponderInterface;

use Config;

/// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and tries to find a appropriate response handler
    pub fn http(
        mut stream: TcpStream,
        socket: SocketAddr,
        config: Config,
        responders: Vec<Box<ResponderInterface + Send>>,
    ) {
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

                            // If we reach the end of the buffer or buffer length exceeds TCP limit
                            if read_size < 512 {
                                break;
                            }
                            if buffer.len() > config.tcp_limit {
                                println!(
                                    "Accumulated buffer {} exceeds size {}, breaking parse",
                                    buffer.len(),
                                    config.tcp_limit
                                );
                                break;
                            }
                        }
                        Err(error) => {
                            println!("Failed to read from TCP stream, error: {}", error);
                            break;
                        }
                    }
                }
            }

            if buffer.len() > 0 {
                // println!("Found non-empty TCP blog {:?} b= {:?}", str::from_utf8(&buffer), buffer);
                let mut response = Vec::new();
                let mut http_dispatcher = http::Dispatcher::new();

                if http_dispatcher.matches(&buffer, &config, &socket) {
                    match http_dispatcher.respond(&buffer, &config, &socket, responders) {
                        Ok(http_response) => {
                            response = http_response;
                        }
                        Err(error) => {
                            println!("Got empty HTTP response! Error: {}", error);
                        }
                    }
                }

                if !response.is_empty() {
                    println!("Found non-empty HTTP response for TCP stream");
                    match stream.write(&response) {
                        Ok(_) => {
                            if let Err(error) = stream.flush() {
                                println!("Failed to flush TCP stream, error: {}", error);
                            }
                        }
                        Err(error) => {
                            println!("Failed to write TCP stream, error: {}", error);
                        }
                    }
                } else {
                    println!(
                        "Found no response for TCP request {:?}",
                        str::from_utf8(&buffer)
                    );
                }
            } else {
                println!("Found empty TCP stream!");
            }
        } else {
            println!("Failed to read initial bytes from TCP stream");
        }
    }
}
