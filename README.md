# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming. 
This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*).

## Goals
* Concurrent web-server with integrated web framework
* Easy to use
* Fast
* Scaleable
* Flexible

## Usage

## Start application from shell

**Rust source**

``` rust
extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::new(Config::from_env()).expect("Failed to start application");
}
```

**Terminal command**

``` bash
$ milstian server port limit
```

## License
This project is under the **GPLv3** license
