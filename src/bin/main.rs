extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::tcp_http(Config::from_env());
}
