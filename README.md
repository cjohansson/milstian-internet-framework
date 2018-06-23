# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming. 
This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*).

## Milestones
* Asynchronous web-server
* Easy to configure
* Fast
* Scaleable

## Usage
*Start application*

``` rust
extern crate milstian;
use milstian::{Application, Config};

...

Application::new(Config {
        limit: 4,
        port: 7878,
        server: String::from("127.0.0.1"),
    });
```

## License
This project is under the **GPLv3** license
