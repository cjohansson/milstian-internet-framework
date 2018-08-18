use std::net::TcpListener;

use Config;
use response::Dispatcher;
use thread::Pool;

pub struct TCP {}

impl TCP {
    /// This method creates a new application based on configuration
    pub fn new(config: Result<Config, String>) -> Result<String, String> {
        let config = config?;
        let path = format!("{}:{}", &config.server_host, &config.server_port);
        let listener = TcpListener::bind(&path);

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.server_limit);

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let config_copy = config.clone();
                            pool.execute(|| {
                                Dispatcher::dispatch_request(stream, config_copy);
                            });
                        }
                        Err(e) => {
                            return Err(format!("Failed to listen to incoming stream, error: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("Failed to bind to server and port, error: {}", e));
            }
        }

        Ok("".to_string())
    }
}
