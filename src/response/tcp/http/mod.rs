//! # TCP HTTP Legacy responders
//! A collection of built-in TCP HTTP responders.

pub mod error;
pub mod file_not_found;
pub mod filesystem;

use std::net::SocketAddr;

use application_layer::http::request;
use application_layer::http::response;

use Application;

pub struct Dispatcher {
    pub request_message: Option<request::Message>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            request_message: None,
        }
    }
}

impl Dispatcher {
    pub fn matches(
        &mut self,
        request: &[u8],
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
    ) -> bool {
        if let Some(request_message) = request::Message::from_tcp_stream(request) {
            self.request_message = Some(request_message);
            return true;
        }
        false
    }

    /// Make the first http response that matches respond
    pub fn respond(
        &self,
        _request: &[u8],
        application: &Application,
        socket: &SocketAddr,
        responders: Vec<Box<ResponderInterface + Send>>,
        overflow_bytes: &u64,
    ) -> Result<(Vec<u8>, String), String> {
        if let Some(request_message) = &self.request_message {
            for mut responder in responders.into_iter() {
                if responder.matches(&request_message, &application, &socket, &overflow_bytes) {
                    if let Ok(mut response) =
                        responder.respond(&request_message, &application, &socket, &overflow_bytes)
                    {
                        let mut log = String::new();
                        if let Some(request_message) = &self.request_message {
                            let mut agent = String::new();
                            let mut referer = String::new();
                            if let Some(http_agent) = request_message.headers.get("User-Agent") {
                                agent = http_agent.to_string();
                            }
                            if let Some(http_referer) = request_message.headers.get("Referer") {
                                referer = http_referer.to_string();
                            }
                            log = format!(
                                "HTTP access - \"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                                socket,
                                &request_message.request_line.raw,
                                agent,
                                referer,
                                &response.status,
                                &response.body.len()
                            );
                        }
                        return Ok((response.to_bytes(), log));
                    }
                }
            }
        }

        return Err("Found no matching HTTP responder".to_string());
    }
}

pub trait ResponderInterface: ResponderInterfaceCopy {
    fn matches(&mut self, &request::Message, &Application, &SocketAddr, &u64) -> bool;
    fn respond(
        &self,
        &request::Message,
        &Application,
        &SocketAddr,
        &u64,
    ) -> Result<response::Message, String>;
}

pub trait ResponderInterfaceCopy {
    fn clone_box(&self) -> Box<ResponderInterface + Send>;
}

impl<T> ResponderInterfaceCopy for T
where
    T: 'static + ResponderInterface + Clone + Send,
{
    fn clone_box(&self) -> Box<ResponderInterface + Send> {
        Box::new(self.clone())
    }
}

impl Clone for Box<ResponderInterface + Send> {
    fn clone(&self) -> Box<ResponderInterface + Send> {
        self.clone_box()
    }
}
