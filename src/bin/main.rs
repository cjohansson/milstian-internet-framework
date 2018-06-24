extern crate milstian;

use std::env;
use std::process;

use milstian::{Application, Config};

fn main() {
    // TODO Add server to shell arguments optionally
    // TODO Add port to shell arguments optionally
    // TODO Add shell arguments optionally

    let config = Config::from_env(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Failed to parse environment arguments: {}", err);
        process::exit(1);
    });

    println!("config: {:?}", config);

    // Application::new(config);
}
