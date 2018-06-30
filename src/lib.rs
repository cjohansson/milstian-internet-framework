mod response;
mod thread;

use std::env;
use std::net::TcpListener;

use response::Dispatcher;
use thread::Pool;

#[derive(Debug)]
pub struct Config {
    pub limit: usize,
    pub port: u32,
    pub server: String,
}

impl Config {
    /// This method takes a vector of strings and creates a config struct
    /// based on index 1 (server), 2 (port) and 3 (limit)
    pub fn from_env_args(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough shell arguments!");
        }
        let limit: usize = match args[3].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse limit!"),
        };
        let port: u32 = match args[2].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse port!"),
        };
        let server = args[1].clone();
        Ok(Config {
            limit,
            port,
            server,
        })
    }

    /// This method collects arguments from environment
    /// and passes them on to method from_env_args
    pub fn from_env() -> Result<Config, &'static str> {
        Config::from_env_args(env::args().collect())
    }
}

#[cfg(test)]
mod config_test {
    use super::*;

    #[test]
    fn from_env_args() {
        // This is expected to work
        let response = Config::from_env_args(vec![
            String::from("ignore this"),
            String::from("127.0.0.1"),
            String::from("7878"),
            String::from("4"),
        ]);
        assert!(response.is_ok());

        // Expected four arguments but received three
        let response = Config::from_env_args(vec![
            String::from("127.0.0.1"),
            String::from("7878"),
            String::from("4"),
        ]);
        assert!(response.is_err());

        // Expected integer but got string
        let response = Config::from_env_args(vec![
            String::from("ignore this"),
            String::from("127.0.0.1"),
            String::from("7878"),
            String::from("coffee"),
        ]);
        assert!(response.is_err());
    }
}

pub struct Application {
    pub config: Config,
}

impl Application {
    /// This method creates a new application based on configuration
    pub fn new(config: Result<Config, &'static str>) -> Result<Application, &'static str> {
        // Parse and validate config
        let config = config?;

        let path = format!("{}:{}", &config.server, &config.port);
        let listener = TcpListener::bind(&path);

        match listener {
            Ok(listener) => {
                let pool = Pool::new(config.limit);

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            pool.execute(|| {
                                Dispatcher::dispatch_request(stream);
                            });
                        }
                        Err(e) => {
                            println!("Failed to listen to incoming stream, error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to bind to server and port, error: {}", e);
            }
        }

        Ok(Application { config })
    }
}
