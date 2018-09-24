extern crate milstian_internet_framework;
use milstian_internet_framework::{Application, Config};
fn main() {
    Application::tcp_http_with_legacy_responders(Config::from_env());
}
