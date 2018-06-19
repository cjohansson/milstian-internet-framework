extern crate webservermulti;
use webservermulti::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / ";
    let sleep = b"GET /sleep ";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("200 OK", "index.htm")

    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("200 OK", "index.htm")
    } else {
        ("404 NOT FOUND", "404.htm")
    };

    // Read file
    let filename = format!("html/{}", filename);
    let mut file = File::open(filename).unwrap();

    // Build response body
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    // Build HTTP response headers
    let mut response_headers = String::new();
    response_headers.push_str(&format!("HTTP/1.1 {}\r\n", status_line));
    response_headers.push_str("Content-Type: text/html\r\n");
    response_headers.push_str("\r\n");

    // Build HTTP response
    let response = format!("{}{}", response_headers, response_body);

    // Flush HTTP response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
