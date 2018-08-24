use std::net::TcpListener;

use response::tcp::Dispatcher;
use thread::Pool;
use Config;

pub struct TCP {}

impl TCP {
    /// This method creates a new application based on configuration
    pub fn new(config: Result<Config, String>) {
        let config = config.expect("Missing configuration!");
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.server_limit);
                loop {
                    match listener.accept() {
                        Ok((stream, socket)) => {
                            let config_copy = config.clone();
                            pool.execute(move || {
                                Dispatcher::dispatch_request(stream, socket, config_copy);
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept a incoming stream, error: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                panic!(format!("Failed to bind to server and port, error: {}", e));
            }
        }
    }
}
