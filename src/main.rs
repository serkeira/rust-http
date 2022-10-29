use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8080";

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_IP, SERVER_PORT)).unwrap();
    println!("Listening on ip: {}, port: {}", SERVER_IP, SERVER_PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let mut response_file = File::open("./src/response.html").unwrap();

    let mut response_file_content = String::new();
    response_file.read_to_string(&mut response_file_content).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", response_file_content.len(), response_file_content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}