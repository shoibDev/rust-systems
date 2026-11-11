use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

// 1: ADD IN MESSAGE FRAMING
// 2: MANAGE EXCEPTION HANDALING
// 3: MANAGE EXCEPTION HANDALING

// Only accounting for the happy path.
fn handle_client(stream: &mut TcpStream) {
    let mut rx_bytes = [0_u8; BUFFER_SIZE];

    loop {
        let n = match stream.read(&mut rx_bytes) {
            Ok(n) if n == 0 => {
                println!("Client Disconnected");
                break;
            }
            Ok(n) => {
                println!("{:?}", rx_bytes);
                n
            }
            Err(e) => {
                eprintln!("An Error has occured: {e}");
                break;
            }
        };

        let received = std::str::from_utf8(&rx_bytes).expect("Valid utf8");
        eprintln!("Received: {}", received);

        // Echo the recieved bytes...
        stream.write_all(received.as_bytes()).expect("...");
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client Connected {}", stream.peer_addr().expect("..."));
                handle_client(&mut stream)
            }
            Err(e) => eprintln!("Failed to connect client, {e}"),
        }
    }
}
