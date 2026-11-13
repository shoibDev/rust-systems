use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

const BUFFER_SIZE: usize = 1024;
const MAX_MESSAGE_SIZE: usize = 1024;

fn handle_client(stream: &mut TcpStream) {
    let mut rx_bytes: [u8; BUFFER_SIZE] = [0_u8; BUFFER_SIZE];
    let mut rx_frame: [u8; 4] = [0_u8; 4];

    'client: loop {
        // Read the 4-byte message length header
        match stream.read_exact(&mut rx_frame) {
            Ok(()) => {}
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => {
                    println!("Client disconnected");
                    break;
                }
                ErrorKind::ConnectionReset => {
                    println!("Client disconnected unexpectedly");
                    break;
                }
                other => {
                    eprintln!("Failed to read header: {} ({:?})", e, other);
                    break;
                }
            },
        }

        let rx_payload_size: u32 = u32::from_be_bytes(rx_frame);

        if rx_payload_size as usize > MAX_MESSAGE_SIZE {
            eprintln!("Received message too large!");
            break;
        }

        // Read the message payload
        let mut bytes_read: u32 = 0;

        while bytes_read < rx_payload_size {
            bytes_read +=
                match stream.read(&mut rx_bytes[bytes_read as usize..rx_payload_size as usize]) {
                    Ok(0) => {
                        println!("Client disconnected");
                        break 'client;
                    }
                    Ok(n) => n as u32,
                    Err(e) => {
                        eprintln!("Failed to read message: {}", e);
                        break 'client;
                    }
                };
        }

        // Convert received bytes to a string
        let received_message = match std::str::from_utf8(&rx_bytes[..rx_payload_size as usize]) {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Invalid UTF-8: {}", e);
                break;
            }
        };

        println!("Received: {}", received_message);

        // Echo the header back to the client
        match stream.write_all(&rx_frame) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to send header: {}", e);
                break;
            }
        }

        // Echo the message back to the client
        match stream.write_all(received_message.as_bytes()) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to send message: {}", e);
                break;
            }
        }

        // Reset buffer
        rx_bytes = [0_u8; BUFFER_SIZE];
    }
}

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    };

    println!("Listening on 127.0.0.1:8080");

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                match stream.peer_addr() {
                    Ok(addr) => println!("Client connected: {}", addr),
                    Err(e) => eprintln!("Failed to get client address: {}", e),
                }

                handle_client(&mut stream);
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
}
