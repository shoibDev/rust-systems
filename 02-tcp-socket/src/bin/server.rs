use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::windows::io::FromRawHandle;

const BUFFER_SIZE: usize = 1024;

// 1: ADD IN MESSAGE FRAMING
// 2: MANAGE EXCEPTION HANDALING
// 3: MANAGE EXCEPTION HANDALING

// Only accounting for the happy path.
fn handle_client(stream: &mut TcpStream) {
    let mut rx_bytes = [0_u8; BUFFER_SIZE];
    let mut frame: [u8; 4] = [0_u8; 4]; // each index repersents one byte of the 4 bytes...payload size is u32...
    let mut bytes_read: u32 = 0;

    'l: loop {
        let y = stream.read_exact(&mut frame);
        println!("{:?}", frame);

        let mut payload_size: u32 = 0;

        for i in frame {
            payload_size += i as u32; // optize and use u32::from_be_bytes...
        }

        if payload_size > rx_bytes.len() as u32 {
            continue 'l;
        }

        while bytes_read < payload_size {
            let n = match stream.read(&mut rx_bytes) {
                Ok(n) if n == 0 => {
                    println!("Client Disconnected");
                    break 'l;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("An Error has occured: {e}");
                    break 'l;
                }
            };
            bytes_read += n as u32;
        }
        let received =
            std::str::from_utf8(&rx_bytes[0..(payload_size as usize)]).expect("Valid utf8");
        eprintln!("Received: {}", received);

        // Echo the recieved bytes...
        stream.write_all(received.as_bytes()).expect("...");
        stream.flush().unwrap();

        // Reset buffer
        rx_bytes = [0_u8; BUFFER_SIZE];
        bytes_read = 0;
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
