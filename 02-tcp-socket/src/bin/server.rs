use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

// Only accounting for the happy path.
fn handle_client(stream: &mut TcpStream) {
    let mut rx_bytes = [0_u8; BUFFER_SIZE];

    loop {
        let n = stream.read(&mut rx_bytes).expect("...");

        let received = std::str::from_utf8(&rx_bytes).expect("Valid utf8");
        eprintln!("n bytes: {}\nMessage: {}", n, received);

        // Echo the recieved bytes...
        stream.write_all(received.as_bytes()).expect("...");
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client Connected");
                handle_client(&mut stream)
            }
            Err(e) => eprintln!("Failed to connect client, {e}"),
        }
    }
}
