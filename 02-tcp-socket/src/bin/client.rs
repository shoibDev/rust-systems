use std::{
    io::{Read, Write, stdin, stdout},
    net::TcpStream,
};

const BUFFER_SIZE: usize = 1024;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server!");

    let mut message: String = String::new();
    let mut rx_bytes = [0_u8; BUFFER_SIZE];

    loop {
        message.clear();

        print!("Message (or /quit): ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut message).expect("It failed...");

        let input = message.trim_end_matches(['\r', '\n']);

        if input == "/quit" {
            break;
        }

        let bytes = input.as_bytes();

        if bytes.len() > BUFFER_SIZE {
            eprintln!("Message too long!");
            continue;
        }

        let header = (bytes.len() as u32).to_be_bytes(); // Try to understrand what BE mans... and did what we did

        stream.write_all(&header);
        stream.write_all(bytes);
        stream.flush().unwrap();

        let n = stream.read(&mut rx_bytes).expect("...");
        let echo: String = std::str::from_utf8(&mut rx_bytes).unwrap().to_string();

        println!("Echo: {echo}");
    }
}
