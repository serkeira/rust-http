use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8080";

fn handle_connection(stream: TcpStream) {
    // TODO
}

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_IP, SERVER_PORT)).unwrap();
    println!("Listening on ip: {}, port: {}", SERVER_IP, SERVER_PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);

        println!("Connected!");
    }
}