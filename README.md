# Milstian - Rust Internet Framework

![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and inspired by the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it
* Websites are scaleable, fast and robust

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
* Run `cargo run localhost 8888 10 index.htm ./html/ 404.htm 1024`

**Parameters are:**
* TCP Hostname
* TCP Port
* Limit of workers
* HTTP directory index file
* HTTP web-server file-system root
* HTTP file not found file
* Maximum TCP request size

## Create application with legacy responders

``` rust
extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::tcp_http_legacy_responders(Config::from_env());
}
```

## Create application with legacy responders and a custom responder

``` rust
extern crate milstian;

use std::net::SocketAddr;

use milstian::application_layer::http::request;
use milstian::response::tcp::http::ResponderInterface;
use milstian::{Application, Config};

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
        _request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> bool {
        false
    }

    fn respond(
        &self,
        _request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> Result<Vec<u8>, String> {
        Err("No result".to_string())
    }
}

fn main() {
    Application::tcp_http_legacy_and_custom_responder(
        Config::from_env(),
        Box::new(Responder::new())
    );
}
```

## HTTP-server benchmark

### Legacy responders

#### Using Apache Benchmark

``` bash
process 1: cargo run localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: ab -n 10000 -c 10 http://localhost:8888/
```

**Expected mean:** 4ms

### Legacy responders with a custom basic dynamic response

TODO

## License

This project is under the **GPLv3** license
