pub mod error;
pub mod file_not_found;
pub mod filesystem;

use std::net::SocketAddr;

use application_layer::http;
use application_layer::http::request;
use Config;

pub struct Dispatcher {
    pub request_message: Option<http::request::Message>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            request_message: None,
        }
    }
}

impl Dispatcher {
    pub fn matches(&mut self, request: &[u8], _config: &Config, _socket: &SocketAddr) -> bool {
        if let Some(request_message) = http::request::Message::from_tcp_stream(request) {
            self.request_message = Some(request_message);
            return true;
        }
        false
    }

    pub fn respond(
        &self,
        _request: &[u8],
        config: &Config,
        _socket: &SocketAddr,
    ) -> Result<Vec<u8>, String> {
        if let Some(request_message) = &self.request_message {
            let mut filesystem = filesystem::Responder::new();
            let mut file_not_found = file_not_found::Responder::new();
            let mut error = error::Responder::new();

            if filesystem.matches(&request_message, &config) {
                return filesystem.respond(&request_message, &config);
            // TODO Add more http response types here: not found, page, ajax, bad request
            } else if file_not_found.matches(&request_message, &config) {
                return file_not_found.respond(&request_message, &config);
            } else if error.matches(&request_message, &config) {
                return error.respond(&request_message, &config);
            }
        }

        return Err("Found no matching HTTP response".to_string());
    }
}

trait ResponderInterface {
    fn new() -> Self;
    fn matches(&mut self, &request::Message, &Config) -> bool;
    fn respond(&self, &request::Message, &Config) -> Result<Vec<u8>, String>;
}
