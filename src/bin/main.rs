extern crate milstian;

use std::net::SocketAddr;

use milstian::application_layer::http::request;
use milstian::response::tcp::http::ResponderInterface;
use milstian::{Application, Config};

#[derive(Clone)]
pub struct Responder {
    pub route: Option<String>,
}

impl Responder {
    pub fn new() -> Responder {
        Responder { route: None }
    }
}

impl ResponderInterface for Responder {
    fn matches(
        &mut self,
        _request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> bool {
        false
    }

    fn respond(
        &self,
        _request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> Result<Vec<u8>, String> {
        Err("No result".to_string())
    }
}

fn main() {
    Application::tcp_http_legacy_and_custom_responder(
        Config::from_env(),
        Box::new(Responder::new())
    );
}
