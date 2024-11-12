use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080!");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connection from: {:?}", stream.peer_addr());

                thread::spawn(move || {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(bytes) => {
                            println!("Received: {:?}", &buffer[..bytes]);
                            
                            match stream.write_all(&buffer[..bytes]) {
                                Ok(()) => {
                                    println!("Response sent");
                                }
                                Err(e) => {
                                    println!("An error occurred while sending the response {}", e)
                                }
                            }

                        }
                        Err(e) => {
                            println!(" {}",e);
                        }
                    }
                });
            }
            Err(e) => {
                println!("An error occurred while maintaining the connection: {}", e);
            }
        }
    }
}
