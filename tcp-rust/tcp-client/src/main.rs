use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Succesfully connected to the server!");

            let message_to_server = "Hello from client";
            stream.write_all(message_to_server.as_bytes()).expect("An error occurred while sending data to the server");
            
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).expect("An error occurred while getting data from the server");
        }
        Err(e) => {
            println!("An error occurred while connecting to the server: {}", e);
        }
    }
}
