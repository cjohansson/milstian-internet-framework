# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming. 
This project is based on the programming exercise "Building a multithreaded web server" from the book "The Rust Programming Language" from "no starch press".

## Milestones
* Asynchronous web-server
* Easy to configure
* Fast
* Scaleable

## Usage
*Start application*

``` rust
    Application::new(Config {
        limit: 4,
        port: 7878,
        server: String::from("127.0.0.1"),
    });
```
