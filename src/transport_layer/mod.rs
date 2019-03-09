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
    /// ```rust,should_panic
    /// use milstian_internet_framework::{Application, Config};
    /// use milstian_internet_framework::response::tcp::http::{error, file_not_found, filesystem, ResponderInterface};
    /// use milstian_internet_framework::transport_layer;
    /// let config = Config::from_env().expect("Failed to get configuration from environment");
    /// let application = Application::new(config);
    /// let responders: Vec<Box<ResponderInterface + Send>> = vec![
    ///     Box::new(filesystem::Responder::new()),
    ///     Box::new(file_not_found::Responder::new()),
    ///     Box::new(error::Responder::new()),
    /// ];
    /// transport_layer::TCP::http(&application, responders)
    /// ```
    // TODO Add example here that does not panic
    pub fn http(application: &Application, responders: Vec<Box<ResponderInterface + Send>>) {
        let config = application.get_config();
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);
        application
            .get_feedback()
            .info(format!("Listening on HTTP requests via TCP to {}", &path));

        match listener {
            Ok(listener) => {
                let pool = Pool::new(&application, config.server_limit);
                loop {
                    match listener.accept() {
                        Ok((stream, socket)) => {
                            application
                                .get_feedback()
                                .info(format!("Received new TCP stream from {}", socket));
                            let application = application.clone();
                            let responders = responders.clone();
                            pool.execute(move || {
                                Dispatcher::http(stream, socket, application, responders);
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
