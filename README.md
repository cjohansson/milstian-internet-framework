# Milstian - A Rust Web Framework

![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-web-framework/master/html/img/logo1-modified.jpg)

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and inspired by the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it
* Websites are scaleable, fast and robust

## Goals
* Concurrent Internet-server with integrated HTTP and HTTP over TLS via TCP/IP web application framework
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

## Create application

``` rust
extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::via_tcp(Config::from_env());
}

```


## License

This project is under the **GPLv3** license
