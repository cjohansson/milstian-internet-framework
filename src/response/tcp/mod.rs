//! # Namespace for TCP responses

pub mod http;

use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::str;

use response::tcp::http::ResponderInterface;

use Application;

/// This struct should handle the dispatching of requests to a specific response type
pub struct Dispatcher {}

impl Dispatcher {
    /// This method takes a TcpStream and tries to find a appropriate response handler
    pub fn http(
        mut stream: TcpStream,
        socket: SocketAddr,
        application: Application,
        responders: Vec<Box<ResponderInterface + Send>>,
    ) {
        // Create a array with 512 elements containing the value 0
        let mut temp_buffer = [0; 512];
        let mut buffer: Vec<u8> = Vec::new();
        let config = application.get_config();

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

            // Did we read the maximum number of bytes and does the limit exceed this?
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
                                application.get_feedback().info(format!(
                                    "TCP stream exceeds size {}, aborting parse",
                                    config.tcp_limit
                                ));
                                break;
                            }
                        }
                        Err(error) => {
                            application
                                .get_feedback()
                                .error(format!("Failed to read from TCP stream, error: {}", error));
                            break;
                        }
                    }
                }
            }

            if buffer.len() > 0 {
                // println!("Found non-empty TCP blog {:?} b= {:?}", str::from_utf8(&buffer), buffer);
                let mut response = Vec::new();
                let mut log = String::new();
                let mut http_dispatcher = http::Dispatcher::new();

                if http_dispatcher.matches(&buffer, &application, &socket) {
                    application
                        .get_feedback()
                        .info(format!("Request could be decoded as HTTP"));
                    match http_dispatcher.respond(&buffer, &application, &socket, responders) {
                        Ok((http_response, http_log)) => {
                            response = http_response;
                            log = http_log;
                            application
                                .get_feedback()
                                .info(format!("Found non-empty HTTP response to TCP stream"));
                        }
                        Err(error) => {
                            application
                                .get_feedback()
                                .error(format!("Got empty HTTP response! Error: {}", error));
                        }
                    }
                } else {
                    application
                        .get_feedback()
                        .info(format!("Request could not be decoded as HTTP"));
                }

                if !response.is_empty() {
                    application
                        .get_feedback()
                        .info(log);
                    match stream.write(&response) {
                        Ok(_) => {
                            if let Err(error) = stream.flush() {
                                application
                                    .get_feedback()
                                    .info(format!("Failed to flush TCP stream, error: {}", error));
                            }
                        }
                        Err(error) => {
                            application
                                .get_feedback()
                                .error(format!("Failed to write to TCP stream, error: {}", error));
                        }
                    }
                } else {
                    application.get_feedback().error(format!(
                        "Found no response for TCP stream {:?}",
                        str::from_utf8(&buffer)
                    ));
                }
            } else {
                application
                    .get_feedback()
                    .info("TCP stream was empty".to_string());
            }
        } else {
            application
                .get_feedback()
                .info("Failed to read from TCP stream".to_string());
        }
    }
}
