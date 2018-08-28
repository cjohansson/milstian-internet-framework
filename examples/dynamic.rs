extern crate milstian;

use std::collections::HashMap;
use std::net::SocketAddr;

use milstian::application_layer::http::request;
use milstian::application_layer::http::response;
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
        request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> bool {
        match request_message.request_line.query_arguments.get("test") {
            Some(value) => {
                self.route = Some(value.clone());
                return true;
            },
            None => {
                return false;
            }
        }
    }

    fn respond(
        &self,
        request_message: &request::Message,
        _config: &Config,
        _socket: &SocketAddr,
    ) -> Result<Vec<u8>, String> {
        if let Some(route) = &self.route {
            let protocol = request::Message::get_protocol_text(
                &request_message.request_line.protocol,
            );
            let mut headers: HashMap<String, String> = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            return Ok(response::Message::new(
                protocol.to_string(),
                "200 OK".to_string(),
                headers,
                format!("Was here: {}", route).as_bytes().to_vec(),
            ).to_bytes());
        } else {
            Err("No result".to_string())
        }
    }
}

fn main() {
    Application::tcp_http_with_legacy_and_custom_responders(
        Config::from_env(),
        Box::new(Responder::new())
    );
}
