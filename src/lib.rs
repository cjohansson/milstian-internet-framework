extern crate chrono;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::net::TcpListener;

mod mime;
mod response;
mod thread;
mod application_layer_protocol;

use response::Dispatcher;
use thread::Pool;

#[derive(Clone, Debug)]
pub struct Config {
    pub file_not_found_file: String,
    pub filesystem_directory_index: String,
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
    pub fn from_env_args(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 6 {
            return Err("Not enough shell arguments!".to_string());
        }
        let server_limit: usize = match args[3].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse limit!".to_string()),
        };
        let server_port: u32 = match args[2].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse port!".to_string()),
        };
        let server_host = args[1].clone();
        let filesystem_directory_index = args[4].clone();
        let mut filesystem_root = args[5].clone();
        let file_not_found_file = args[6].clone();
        let root_path = PathBuf::from(&filesystem_root);
        match fs::canonicalize(root_path) {
            Ok(canonical_root) =>  {
                if let Some(canonical_root) = canonical_root.to_str() {
                    filesystem_root = canonical_root.to_string();
                } else {
                    return Err(format!("Failed to convert canonical root to string {:?}", canonical_root));
                }
            },
            Err(error) => { return Err(format!("Could not find canonical path from: {}, error: {}", &filesystem_root, &error));
            }
        }
        Ok(Config {
            filesystem_directory_index,
            file_not_found_file,
            filesystem_root,
            server_limit,
            server_host,
            server_port,
        })
    }

    /// This method collects arguments from environment
    /// and passes them on to method from_env_args
    pub fn from_env() -> Result<Config, String> {
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
            String::from("404.htm"),
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
            String::from("4"),
            String::from("index.htm"),
            String::from("./htmls/"),
            String::from("404.htm"),
        ]);
        assert!(response.is_err());
    }
}

pub struct Application;

impl Application {
    /// This method creates a new application based on configuration
    pub fn new(config: Result<Config, String>) -> Result<Application, String> {
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
                            eprintln!("Failed to listen to incoming stream, error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to bind to server and port, error: {}", e);
            }
        }

        Ok(Application)
    }
}
