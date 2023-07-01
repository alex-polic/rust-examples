use std::{net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}, io::{Read, Write}, thread};


fn main() {
    let address = Ipv4Addr::new(127,0,0,1);
    let socket = SocketAddrV4::new(address, 3001);
    let listener = TcpListener::bind(socket)
    .expect("Something went wrong, could not bind socket to TCP listener");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                thread::spawn( move || {
                    handle_client_request(tcp_stream)
                });
            },
            Err(_) => println!("Error connection client"),
        }
    }; 
}

fn handle_client_request(mut tcp_stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = tcp_stream.read(&mut buffer); // Read the incoming request into the buffer

    let request = String::from_utf8_lossy(&buffer[..]); // Convert the buffer to a string
    println!("Received request:\n{}", request);

    // Check if the request is in the expected format
    if request.starts_with("GET /hello HTTP/1.1\r\n") {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, world!";
        let _ = tcp_stream.write(response.as_bytes()); // Send the response back to the client
        println!("Sent response:\n{}", response);
    } else {
        let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request";
        let _ = tcp_stream.write(response.as_bytes()); // Send a "Bad Request" response
        println!("Sent response:\n{}", response);
    }
}