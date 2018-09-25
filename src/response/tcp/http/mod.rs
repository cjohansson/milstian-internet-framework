//! # TCP/IP HTTP Legacy responders
//! A collection of built-in TCP/IP HTTP responders.

pub mod error;
pub mod file_not_found;
pub mod filesystem;

use std::net::SocketAddr;

extern crate milstian_http;
use milstian_http::request;

use Config;

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
    pub fn matches(&mut self, request: &[u8], _config: &Config, _socket: &SocketAddr) -> bool {
        if let Some(request_message) = request::Message::from_tcp_stream(request) {
            self.request_message = Some(request_message);
            return true;
        }
        false
    }

    pub fn respond(
        &self,
        _request: &[u8],
        config: &Config,
        socket: &SocketAddr,
        responders: Vec<Box<ResponderInterface + Send>>,
    ) -> Result<Vec<u8>, String> {
        if let Some(request_message) = &self.request_message {
            for mut responder in responders.into_iter() {
                if responder.matches(&request_message, &config, &socket) {
                    return responder.respond(&request_message, &config, &socket);
                }
            }
        }

        return Err("Found no matching HTTP response".to_string());
    }
}

pub trait ResponderInterface: ResponderInterfaceCopy {
    fn matches(&mut self, &request::Message, &Config, &SocketAddr) -> bool;
    fn respond(&self, &request::Message, &Config, &SocketAddr) -> Result<Vec<u8>, String>;
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
