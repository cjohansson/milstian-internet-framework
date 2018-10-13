# Milstian - a Rust Internet Framework

![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and inspired by the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it that is scaleable, fast and robust

## Goals
* Concurrent Internet-server with integrated HTTP and HTTP over TLS via TCP/IP support
* Integrated web application framework
* Easy to customize for any kind of application
* Fast
* Scaleable
* Flexible
* Potential support for other transport protocols and application layer protocols

## Development

* Use `rust-fmt` on all rust files
* Use `cargo check` and `cargo test` to ensure validity

## Run local server

* visit project repository root
* Run `cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024`

**Parameters are:**
* TCP Hostname
* TCP Port
* Limit of workers
* HTTP directory index file
* HTTP web-server file-system root
* HTTP file not found file
* Maximum TCP request size

## Example static TCP-HTTP application

``` rust
extern crate milstian_internet_framework;
use milstian_internet_framework::{Application, Config};

fn main() {
    let config = Config::from_env().expect("Failed to get configuration from environment");
    Application::new(config).tcp_http_with_legacy_responders();
}
```

## Example simple dynamic TCP-HTTP application

``` rust
extern crate milstian_internet_framework;

use std::collections::HashMap;
use std::net::SocketAddr;
use milstian_internet_framework::request;
use milstian_internet_framework::response;
use milstian_internet_framework::response::tcp::http::ResponderInterface;
use milstian_internet_framework::{Application, Config};

#[derive(Clone)]
pub struct Responder {
    pub route: Option<String>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder { route: None }
    }
}

impl ResponderInterface for Responder {
    fn matches(
        &mut self,
        request_message: &request::Message,
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u8,
    ) -> bool {
        match request_message.request_line.query_arguments.get("test") {
            Some(value) => {
                self.route = Some(value.clone());
                return true;
            }
            None => {
                return false;
            }
        }
    }

    fn respond(
        &self,
        request_message: &request::Message,
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u8,
    ) -> Result<Vec<u8>, String> {
        if let Some(route) = &self.route {
            let protocol =
                request::Message::get_protocol_text(&request_message.request_line.protocol);
            let mut headers: HashMap<String, String> = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            return Ok(response::Message::new(
                protocol.to_string(),
                "200 OK".to_string(),
                headers,
                format!("Was here: {}", route).as_bytes().to_vec(),
            ).to_bytes());
        } else {
            Err("No result".to_string())
        }
    }
}

fn main() {
    let config = Config::from_env().expect("Failed to get configuration from environment");
    Application::new(config).tcp_http_with_legacy_and_custom_responders(Box::new(Responder::new()));
}
```

## Docs

* [Benchmark](docs/BENCHMARK.md)
* [Issues](docs/ISSUES.md)

## License

This project is under the **GPLv3** license
