# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and inspired by the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it
* Websites are scaleable, fast and robust

## Goals
* Concurrent TCP-server with integrated web application framework and potential support for other application layer protocols.
* Easy to customize for any kind of application
* Fast
* Scaleable
* Flexible

## Development

* Use `rust-fmt` on all rust files
* Use `cargo check` and `cargo test` to ensure validity

## Run local server

`cargo run localhost 8888 10 index.htm ../html/ 404.htm`

**Parameters are:**
* Hostname
* Port
* Limit of workers
* Directory index file
* HTTP web-server file-system root
* File not found file

## Run tests

`cargo test`

## License

This project is under the **GPLv3** license
