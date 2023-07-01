use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

use crate::http_parser;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3001").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:3001");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                thread::spawn(move || {
                    handle_client_request(tcp_stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client_request(mut tcp_stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = tcp_stream.read(&mut buffer);

    let request_text = String::from_utf8_lossy(&buffer[..]);

    let request = http_parser::read_http1x_request(&request_text.to_string());

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, world!";
    let _ = tcp_stream.write(response.as_bytes());
}

// ... Rest of the code for reading HTTP requests

