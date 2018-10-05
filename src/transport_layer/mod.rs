//! # Supported transport layers
//! Binds to the transport layer socket and spawns new threads for dispatching responses.

extern crate chrono;
use chrono::offset::Utc;
use std::net::TcpListener;

use response::tcp::http::ResponderInterface;
use response::tcp::Dispatcher;
use thread::Pool;
use Config;

pub struct TCP {}

impl TCP {
    /// This method creates a new HTTP over TCP application based on configuration
    /// ```rust
    /// let responders: Vec<Box<ResponderInterface + Send>> = vec![
    ///     Box::new(filesystem::Responder::new()),
    ///     Box::new(file_not_found::Responder::new()),
    ///     Box::new(error::Responder::new()),
    /// ];
    /// transport_layer::TCP::http(config, responders)
    /// ```
    // TODO Add example here
    pub fn http(config: Result<Config, String>, responders: Vec<Box<ResponderInterface + Send>>) {
        let config = config.expect("Missing configuration!");
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);
        println!("Starting listening on TCP/IP connections to: {}", &path);

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.server_limit);
                loop {
                    match listener.accept() {
                        Ok((stream, socket)) => {
                            println!(
                                "{} - new TCP packet",
                                Utc::now().format("%Y-%m-%d %H:%M:%S")
                            );
                            let config = config.clone();
                            let responders = responders.clone();
                            pool.execute(move || {
                                Dispatcher::http(stream, socket, config, responders);
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept a incoming stream, error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                panic!(format!("Failed to bind to server and port: {}, error: {}", &path, e));
            }
        }
    }
}
