use std::net::TcpListener;

use response::tcp::http::ResponderInterface;
use response::tcp::Dispatcher;
use thread::Pool;
use Config;

pub struct TCP {}

impl TCP {
    /// This method creates a new application based on configuration
    pub fn http(config: Result<Config, String>, responders: Vec<Box<ResponderInterface + Send>>) {
        let config = config.expect("Missing configuration!");
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.server_limit);
                loop {
                    match listener.accept() {
                        Ok((stream, socket)) => {
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
                panic!(format!("Failed to bind to server and port, error: {}", e));
            }
        }
    }
}
