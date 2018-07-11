use std::env;
use std::net::TcpListener;

mod response;
mod thread;

use response::Dispatcher;
use thread::Pool;

#[derive(Clone, Debug)]
pub struct Config {
    pub filesystem_index: String,
    pub filesystem_root: String,
    pub server_limit: usize,
    pub server_host: String,
    pub server_port: u32,
}

// TODO Add a TOML parser

// TODO Add command line parser

impl Config {
    /// This method takes a vector of strings and creates a config struct
    /// based on index 1 (server), 2 (port) and 3 (limit)
    pub fn from_env_args(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("Not enough shell arguments!");
        }
        let server_limit: usize = match args[3].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse limit!"),
        };
        let server_port: u32 = match args[2].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse port!"),
        };
        let server_host = args[1].clone();
        let filesystem_index = args[4].clone();
        let filesystem_root = args[5].clone();
        Ok(Config {
            filesystem_index,
            filesystem_root,
            server_limit,
            server_host,
            server_port,
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
            String::from("index.htm"),
            String::from("./html/"),
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
            String::from("index.htm"),
            String::from("./html/"),
        ]);
        assert!(response.is_err());
    }
}

pub struct Application;

impl Application {
    /// This method creates a new application based on configuration
    pub fn new(config: Result<Config, &'static str>) -> Result<Application, &'static str> {
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
                            println!("Failed to listen to incoming stream, error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to bind to server and port, error: {}", e);
            }
        }

        Ok(Application)
    }
}
