extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::via_tcp(Config::from_env());
}
