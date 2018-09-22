extern crate milstian;

use std::collections::HashMap;
use std::net::SocketAddr;

extern crate milstian_http;
use milstian_http::request;
use milstian_http::request::BodyContentType;
use milstian_http::response;

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
            }
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
            let protocol =
                request::Message::get_protocol_text(&request_message.request_line.protocol);
            let mut headers: HashMap<String, String> = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            let upload = match request_message.body {
                BodyContentType::MultiPart(ref body) => match body.get(&"file".to_string()) {
                    Some(value) => match String::from_utf8(value.body.clone()) {
                        Ok(utf8_value) => utf8_value,
                        _ => format!("no UTF-8 file data in: {:?}", &value.body),
                    },
                    _ => format!("no file data in {:?}", request_message),
                },
                _ => "no data".to_string(),
            };

            let output = format!("<html><head><title>Milstian Web Framework - Dynamic Test</title><link rel='stylesheet' href='/css/style.css' /></head><body><div class='wrapper'><h1>Milstian Web Framework</h1><img alt='' src='/img/logo1-modified.jpg' /><p><strong>Query argument:</strong> {}</p><div><strong>File upload:</strong><br /><pre>{}</pre></div><h2>Dynamic Test</h2><form action='' method='post' enctype='multipart/form-data'><fieldset><legend>File upload</legend><div><label>Select file<br /><input type='file' name='file' /></label></div><div><input type='submit' value='Upload' /></div></fieldset></form></div></body></html>", route, &upload);

            return Ok(response::Message::new(
                protocol.to_string(),
                "200 OK".to_string(),
                headers,
                output.as_bytes().to_vec(),
            ).to_bytes());
        } else {
            Err("No result".to_string())
        }
    }
}

fn main() {
    Application::tcp_http_with_legacy_and_custom_responders(
        Config::from_env(),
        Box::new(Responder::new()),
    );
}
