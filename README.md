# Milstian - A Rust Web Framework

In progress, primarily used for learning Rust programming.

This project is based on the programming exercise *Building a multithreaded web server* from the book *The Rust Programming Language* (*no starch press 2018*) and the *Aomebo Web Framework for PHP*.

## Major goal
* Easy to make any kind of website with it
* Website are scaleable, fast and robust

## Goals
* Concurrent TCP-server with integrated HTTP application framework and potential support for other transport-protocols.
* Easy to customize for any kind of application
* Fast
* Scaleable
* Flexible

## Development

* Use `rust-fmt`, an Emacs hook is included via DirectoryVariables.

## Run local server

`cargo run localhost 8888 10 index.htm ../html/ 404.htm`

**Parameters are:**
* Hostname
* Port
* Worker limit
* Directory index file
* HTTP web-server file-system root
* File not found file

## Run tests

`cargo test`

## License
This project is under the **GPLv3** license
