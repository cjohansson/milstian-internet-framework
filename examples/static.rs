extern crate milstian_internet_framework;
use milstian_internet_framework::{Application, Config};
fn main() {
    let config = Config::from_env().expect("Failed to get configuration from environment");
    Application::new(config).tcp_http_with_legacy_responders();
}
