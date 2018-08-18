extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::from_tcp(Config::from_env()).expect("Failed to start application");
}
