extern crate milstian;

use milstian::response::tcp::http::{error, file_not_found, filesystem, ResponderInterface};
use milstian::{Application, Config};

fn main() {
    let responders: Vec<Box<ResponderInterface + Send>> = vec![
        Box::new(filesystem::Responder::new()),
        Box::new(file_not_found::Responder::new()),
        Box::new(error::Responder::new()),
    ];

    Application::tcp_http(Config::from_env(), responders);
}
