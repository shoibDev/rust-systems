use std::io::Read;
use std::net::{TcpListener, TcpStream};

const MESSAGE_SIZE: usize = 5;

fn handle_client(stream: &mut TcpStream) {
    println!("Client Connected...");

    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    stream.read(&mut rx_bytes).unwrap();

    let received = std::str::from_utf8(&rx_bytes).expect("Valid utf8");
    eprintln!("{}", received);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_client(&mut stream),
            Err(e) => eprintln!("Failed to connect client, {e}"),
        }
    }
}
