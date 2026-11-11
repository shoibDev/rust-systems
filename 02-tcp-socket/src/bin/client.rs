use std::{
    io::{ErrorKind, Read, Write, stdin, stdout},
    net::TcpStream,
};

const BUFFER_SIZE: usize = 1024;
const MAX_MESSAGE_SIZE: usize = 1024;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => {
            match stream.peer_addr() {
                Ok(addr) => println!("Connected to Peer: {}", addr),
                Err(e) => match e.kind() {
                    ErrorKind::NotConnected => {
                        eprintln!("Stream is not connected");
                    }
                    other => {
                        eprintln!("Unexpected: {} ({:?})", e, other);
                    }
                },
            }
            stream
        }

        Err(e) => match e.kind() {
            ErrorKind::ConnectionRefused => {
                eprintln!("Connection refused - no service listening on that port");
                std::process::exit(1)
            }
            ErrorKind::TimedOut => {
                eprintln!("Connection timed out - firewall or server overload");
                std::process::exit(1)
            }
            other => {
                eprintln!("Unexpected error: {} ({:?})", e, other);
                std::process::exit(1)
            }
        },
    };

    let mut rx_bytes: [u8; BUFFER_SIZE] = [0_u8; BUFFER_SIZE];
    let mut rx_frame: [u8; 4] = [0_u8; 4];

    loop {
        let mut raw_input: String = String::new();

        print!("Message (or /quit): ");
        match stdout().flush() {
            Ok(()) => {}
            Err(e) => eprintln!("Failed to flush stdout: {}", e),
        }

        match stdin().read_line(&mut raw_input) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                continue;
            }
        };

        let cleaned_input = raw_input.trim_end_matches(['\r', '\n']);
        if cleaned_input == "/quit" {
            break;
        }

        let message_bytes: &[u8] = cleaned_input.as_bytes();
        if message_bytes.len() > MAX_MESSAGE_SIZE {
            eprintln!("Message too long!");
            continue;
        }

        let message_length = message_bytes.len() as u32;
        let length_header = message_length.to_be_bytes();

        match stream.write_all(&length_header) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to send header: {e}");
                break;
            }
        }

        match stream.write_all(message_bytes) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to send message: {e}");
                break;
            }
        }

        stream.read_exact(&mut rx_frame).expect("...");

        let echo_payload_size: u32 = u32::from_be_bytes(rx_frame);

        let mut bytes_read: u32 = 0;
        while echo_payload_size < message_length {
            let n = stream.read(&mut rx_bytes).expect("...");
            bytes_read += n as u32;
        }

        println!("{:?}", rx_bytes);

        let echo: String = std::str::from_utf8(&mut rx_bytes).unwrap().to_string();

        println!("Echo: {echo}");

        // reset rx_bytes
        rx_bytes = [0_u8; BUFFER_SIZE];
    }
}
