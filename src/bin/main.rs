use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use simple_rust_webserver::ThreadPool;

const GET_INDEX: &'static str = "GET / HTTP/1.1\r\n";
const ANOTHER_PATH: &'static str = "GET /another HTTP/1.1\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let tpool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        tpool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let result = stream.read(&mut buffer);
    match result {
        Ok(_) => {
            println!(
                "Connection established\r\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }
        Err(e) => {
            println!("{:#?}", e.to_string());
            panic!()
        }
    }

    let (status_line, filename) = if buffer.starts_with(GET_INDEX.as_bytes()) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(ANOTHER_PATH.as_bytes()) {
        ("HTTP/1.1 200 OK", "another.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
