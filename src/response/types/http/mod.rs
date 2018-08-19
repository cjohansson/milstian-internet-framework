pub mod file_not_found;
pub mod filesystem;

use application_layer::http;
use response::Type;
use Config;

pub struct Dispatcher {
    pub request_message: Option<http::request::Message>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher { request_message: None }
    }
}

impl Type<Dispatcher> for Dispatcher {
    fn matches(&mut self, request: &[u8], _config: &Config) -> bool {
        if let Some(request_message) = http::request::Message::from_tcp_stream(request) {
            self.request_message = Some(request_message);
            return true;
        }
        false
    }

    fn respond(&self, _request: &[u8], config: &Config) -> Result<Vec<u8>, String> {
        if let Some(request_message) = &self.request_message {
            let mut filesystem = filesystem::Responder::new();
            let mut file_not_found = file_not_found::Responder::new();

            if filesystem.matches(&request_message, &config) {
                return filesystem.respond(&request_message, &config);
            } else if file_not_found.matches(&request_message, &config) {
                return file_not_found.respond(&request_message, &config);
            }
            // TODO Add more http response types here: not found, page, ajax, bad request
        }

        return Err("Found no matching HTTP response".to_string());
    }
}
