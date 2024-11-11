use std::io::{Read, Write};
use std::net::{TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080!");
}
