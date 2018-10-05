//! # Milstian Internet Framework
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)
//!
//! In progress, primarily used for learning Rust programming.
//!
//! This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and inspired by the *Aomebo Web Framework for PHP*.
//!
//! ## Major goal
//! * Easy to make any kind of website with it that is scaleable, fast and robust
//!
//! ## Usage
//! This crate is on [crates.io](https://crates.io/crates/milstian-internet-framework) and can be used by adding time to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! milstian_internet_framework = "0.1.*"
//! ```
//! And this in your crate root:
//! ```rust,dont_run
//! extern crate milstian_internet_framework;
//! ```

extern crate milstian_feedback;
extern crate milstian_http;

pub mod application_layer;
pub mod mime;
pub mod response;
mod thread;
pub mod transport_layer;

extern crate chrono;

use std::env;
use std::fs;
use std::path::PathBuf;

use milstian_feedback::Feedback;
use response::tcp::http::{error, file_not_found, filesystem, ResponderInterface};

#[derive(Clone, Debug)]
/// # Holds application configuration, can be created in different ways.
/// ## From environment:
/// ```rust
///use milstian_internet_framework::Config;
///let config = Config::from_env();
///assert!(config.is_err()); // Expected fail since environment variables is missing
/// ```
pub struct Config {
    pub feedback_error_file: Option<String>,
    pub feedback_info_file: Option<String>,
    pub file_not_found_file: String,
    pub filesystem_directory_index: String,
    pub filesystem_root: String,
    pub server_limit: usize,
    pub server_host: String,
    pub server_port: u32,
    pub tcp_limit: usize,
}

impl Config {
    /// Find canonical root from a string path
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

    /// This method takes a vector of strings and creates a config struct based on argument vector
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
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index,
            file_not_found_file,
            filesystem_root,
            server_limit,
            server_host,
            server_port,
            tcp_limit,
        })
    }

    /// This method collects arguments from environment and passes them on to method from_env_args
    /// # Example
    /// ```rust
    /// use milstian_internet_framework::Config;
    /// let config = Config::from_env();
    /// ```
    pub fn from_env() -> Result<Config, String> {
        Config::from_env_args(env::args().collect())
    }
}

/// # Main entry point for a new application.
/// Could in the future support multiple transport layers and application layers.
/// ## TCP/IP HTTP static application:
/// ```rust,should_panic
/// use milstian_internet_framework::{Application, Config};
/// Application::tcp_http_with_legacy_responders(Config::from_env());
/// ```
#[derive(Clone, Debug)]
pub struct Application {
    config: Config,
    feedback: Feedback,
}

impl Application {
    pub fn new(config: Config) -> Application {
        Application {
            config,
            feedback: Feedback::new(config.feedback_error_file, config.feedback_info_file),
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_feedback(&self) -> &Feedback {
        &self.feedback
    }

    /// Create a new TCP/IP HTTP application
    /// # Example
    /// ```rust,should_panic
    /// extern crate milstian_internet_framework;
    /// use milstian_internet_framework::{Application, Config};
    /// use milstian_internet_framework::response::tcp::http::{error, file_not_found, filesystem, ResponderInterface};
    /// fn main() {
    ///     let responders: Vec<Box<ResponderInterface + Send>> = vec![
    ///         Box::new(filesystem::Responder::new()),
    ///         Box::new(error::Responder::new()),
    ///     ];
    ///     Application::tcp_http(Config::from_env(), responders);
    /// }
    /// ```
    // TODO Use example that doesn't panic
    pub fn tcp_http(&self, responders: Vec<Box<ResponderInterface + Send>>) {
        transport_layer::TCP::http(&self, responders)
    }

    /// Create a new TCP/IP HTTP application with the legacy responders
    /// # Example
    /// ```rust,should_panic
    /// extern crate milstian_internet_framework;
    /// use milstian_internet_framework::{Application, Config};
    /// fn main() {
    ///     Application::tcp_http_with_legacy_responders(Config::from_env());
    /// }
    /// ```
    // TODO Use example that doesn't panic
    pub fn tcp_http_with_legacy_responders(&self) {
        let responders: Vec<Box<ResponderInterface + Send>> = vec![
            Box::new(filesystem::Responder::new()),
            Box::new(file_not_found::Responder::new()),
            Box::new(error::Responder::new()),
        ];
        transport_layer::TCP::http(&self, responders)
    }

    /// # Create a new TCP/IP with legacy and a custom responder
    /// ```rust,should_panic
    /// use milstian_internet_framework::{Application, Config};
    /// fn main() {
    ///     Application::tcp_http_with_legacy_responders(Config::from_env());
    /// }
    /// ```
    // TODO Use example that doesn't panic
    pub fn tcp_http_with_legacy_and_custom_responders(
        &self,
        custom: Box<ResponderInterface + Send>,
    ) {
        let responders: Vec<Box<ResponderInterface + Send>> = vec![
            custom,
            Box::new(filesystem::Responder::new()),
            Box::new(file_not_found::Responder::new()),
            Box::new(error::Responder::new()),
        ];
        transport_layer::TCP::http(&self, responders)
    }
}

#[cfg(test)]
mod tests {
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
