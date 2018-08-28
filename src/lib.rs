pub mod application_layer;
pub mod mime;
pub mod response;
mod thread;
pub mod transport_layer;

extern crate chrono;

use std::env;
use std::fs;
use std::path::PathBuf;

use response::tcp::http::{error, file_not_found, filesystem, ResponderInterface};

#[derive(Clone, Debug)]
pub struct Config {
    pub file_not_found_file: String,
    pub filesystem_directory_index: String,
    pub filesystem_root: String,
    pub server_limit: usize,
    pub server_host: String,
    pub server_port: u32,
    pub tcp_limit: usize,
}

// TODO Add a TOML parser
// TODO Add command line parser

impl Config {
    pub fn get_canonical_root(root_path: &String) -> Result<String, String> {
        let root_path = PathBuf::from(&root_path);
        match fs::canonicalize(&root_path) {
            Ok(canonical_root) => {
                if let Some(canonical_root) = canonical_root.to_str() {
                    return Ok(canonical_root.to_string());
                } else {
                    return Err(format!(
                        "Failed to convert canonical root to string {:?}",
                        canonical_root
                    ));
                }
            }
            Err(error) => {
                return Err(format!(
                    "Could not find canonical path from: {:?}, error: {}",
                    &root_path, &error
                ));
            }
        }
    }

    /// This method takes a vector of strings and creates a config struct
    /// based on index 1 (server), 2 (port) and 3 (limit)
    pub fn from_env_args(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 8 {
            return Err("Not enough shell arguments!".to_string());
        }
        let server_limit: usize = match args[3].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse server limit!".to_string()),
        };
        let server_port: u32 = match args[2].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse server port!".to_string()),
        };
        let server_host = args[1].clone();
        let filesystem_directory_index = args[4].clone();
        let filesystem_root = Config::get_canonical_root(&args[5])?;
        let file_not_found_file = args[6].clone();
        let tcp_limit: usize = match args[7].clone().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse TCP limit!".to_string()),
        };
        Ok(Config {
            filesystem_directory_index,
            file_not_found_file,
            filesystem_root,
            server_limit,
            server_host,
            server_port,
            tcp_limit,
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
            String::from("1024"),
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
    pub fn tcp_http(
        config: Result<Config, String>,
        responders: Vec<Box<ResponderInterface + Send>>,
    ) {
        transport_layer::TCP::http(config, responders)
    }

    pub fn tcp_http_with_legacy_responders(
        config: Result<Config, String>
    ) {
        let responders: Vec<Box<ResponderInterface + Send>> = vec![
            Box::new(filesystem::Responder::new()),
            Box::new(file_not_found::Responder::new()),
            Box::new(error::Responder::new()),
        ];
        transport_layer::TCP::http(config, responders)
    }

    pub fn tcp_http_with_legacy_and_custom_responders(
        config: Result<Config, String>,
        custom: Box<ResponderInterface + Send>
    ) {
        let responders: Vec<Box<ResponderInterface + Send>> = vec![
            custom,
            Box::new(filesystem::Responder::new()),
            Box::new(file_not_found::Responder::new()),
            Box::new(error::Responder::new()),
        ];
        transport_layer::TCP::http(config, responders)
    }

}
