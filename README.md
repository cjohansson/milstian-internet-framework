# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming. 
This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*).

## Milestones
* Asynchronous web-server
* Easy to configure
* Fast
* Scaleable

## Usage

## Start application from shell

``` rust
extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::new(Config::from_env()).expect("Failed to start application");
}
```

``` bash
executable server port limit
```

## License
This project is under the **GPLv3** license
