//! # Supported transport layers
//! Binds to the transport layer socket and spawns new threads for dispatching responses.

use std::net::TcpListener;

use response::tcp::http::ResponderInterface;
use response::tcp::Dispatcher;
use thread::Pool;
use Application;

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
    pub fn http(application: Application, responders: Vec<Box<ResponderInterface + Send>>) {
        let config = application.get_config();
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);
        application.get_feedback().info(format!(
            "Starting listening on TCP/IP connections to: {}",
            &path
        ));

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.server_limit);
                loop {
                    match listener.accept() {
                        Ok((stream, socket)) => {
                            application.get_feedback().info(format!(
                                "Received new TCP/IP stream from {}",
                                socket
                            ));
                            let config = config.clone();
                            let responders = responders.clone();
                            pool.execute(move || {
                                Dispatcher::http(stream, socket, config, responders);
                            });
                        }
                        Err(e) => {
                            application
                                .get_feedback()
                                .error(format!("Failed to accept a incoming stream, error: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                panic!(format!(
                    "Failed to bind to server and port: {}, error: {}",
                    &path, e
                ));
            }
        }
    }
}
