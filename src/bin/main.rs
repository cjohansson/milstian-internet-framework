extern crate webservermulti;
use webservermulti::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {

    // TODO Add server to shell arguments optionally
    // TODO Add port to shell arguments optionally
    // TODO Add shell arguments optionally
    let server = "127.0.0.1";
    let port = "7878";
    let limit = 4;

    let path = format!("{}:{}", &server, &port);
    let listener = TcpListener::bind(&path);

    match listener {
        Ok(listener) => {
            let pool = ThreadPool::new(limit);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        pool.execute(|| {
                            handle_connection(stream);
                        });
                    },
                    Err(e) => {
                        println!("Failed to listen to incoming stream, error: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to bind to server and port, error: {}", e);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // TODO Handle this unwrap
    stream.read(&mut buffer).unwrap();

    let get = b"GET / ";
    let sleep = b"GET /sleep ";

    // TODO Make these more dynamic
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

    // TODO Handle this unwrap
    // TODO Make the path more dynamic
    let mut file = File::open(filename).unwrap();

    // Build response body
    let mut response_body = String::new();

    // TODO Handle this unwrap
    file.read_to_string(&mut response_body).unwrap();

    // TODO Make these more dynamic
    // Build HTTP response headers
    let mut response_headers = String::new();
    response_headers.push_str(&format!("HTTP/1.1 {}\r\n", status_line));
    response_headers.push_str("Content-Type: text/html\r\n");

    // TODO Add more headers here
    response_headers.push_str("\r\n");

    // Build HTTP response
    let response = format!("{}{}", response_headers, response_body);

    // Flush HTTP response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
