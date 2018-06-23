extern crate milstian;
use milstian::{Application, Config};

fn main() {
    // TODO Add server to shell arguments optionally
    // TODO Add port to shell arguments optionally
    // TODO Add shell arguments optionally
    let server = String::from("127.0.0.1");
    let port = 7878;
    let limit = 4;

    Application::new(Config {
        limit,
        port,
        server,
    });

}
