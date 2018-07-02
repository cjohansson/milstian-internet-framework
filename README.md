# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it, that is scaleable and fast

## Goals
* Concurrent TCP-server with integrated HTTP application framework
* Easy to customize for any kind of application
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

## Development

* Use rust-fmt

## License
This project is under the **GPLv3** license
