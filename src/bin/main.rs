extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::tcp_http_legacy_responders(Config::from_env());
}
