extern crate milstian_internet_framework;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use milstian_internet_framework::application_layer::http::request;
use milstian_internet_framework::application_layer::http::request::BodyContentType;
use milstian_internet_framework::application_layer::http::response;
use milstian_internet_framework::response::tcp::http::ResponderInterface;
use milstian_internet_framework::{Application, Config};

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
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
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
        _application: &Application,
        _socket: &SocketAddr,
        overflow_bytes: &u64,
    ) -> Result<response::Message, String> {
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

            let upload2 = match request_message.body {
                BodyContentType::MultiPart(ref body) => match body.get(&"file2".to_string()) {
                    Some(value) => match String::from_utf8(value.body.clone()) {
                        Ok(utf8_value) => utf8_value,
                        _ => format!("no UTF-8 file data in: {:?}", &value.body),
                    },
                    _ => format!("no file data in {:?}", request_message),
                },
                _ => "no data".to_string(),
            };

            thread::sleep(Duration::from_secs(2));

            let mut overflow_upload = "Upload did not overflow server byte limit!";
            if overflow_bytes > &0 {
                overflow_upload = "Your upload exceeded server byte limit!";
            }

            let output = format!("<html><head><title>Milstian Internet Framework - Dynamic Example</title><link rel='stylesheet' href='/css/style.css' /></head><body><div class='wrapper'><h1>Milstian Web Framework</h1><img alt='' src='/img/logo1-modified.jpg' /><p><strong>Query argument:</strong> {}</p><div><strong>File upload 1:</strong><br /><pre>{}</pre></div><div><strong>File upload 2:</strong><br /><pre>{}</pre></div><h2>Dynamic Test</h2><form action='' method='post' enctype='multipart/form-data'><fieldset><legend>File upload</legend><div><label>Select file 1<br /><input type='file' name='file' /></label></div><div><label>Select file 2<br /><input type='file' name='file2' /></label></div><p>{}</p><div><input type='submit' value='Upload' /></div></fieldset></form></div></body></html>", route, &upload, &upload2, &overflow_upload);

            return Ok(response::Message::new(
                protocol.to_string(),
                "200 OK".to_string(),
                headers,
                output.as_bytes().to_vec(),
            ));
        } else {
            Err("No result".to_string())
        }
    }
}

fn main() {
    let config = Config::from_env().expect("Failed to get configuration from environment");
    Application::new(config).tcp_http_with_legacy_and_custom_responders(Box::new(Responder::new()));
}
