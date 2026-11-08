use std::{
    io::{Read, Write, stdin},
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
        stdin().read_line(&mut message).expect("It failed...");

        let input = message.trim_end_matches(['\r', '\n']);

        if message == "/quit" {
            break;
        }

        let bytes = input.as_bytes();
        stream.write_all(bytes);
        stream.flush().unwrap();

        let n = stream.read(&mut rx_bytes).expect("...");
        let echo: String = std::str::from_utf8(&mut rx_bytes).unwrap().to_string();

        println!("Echo: {echo}");
    }
}
