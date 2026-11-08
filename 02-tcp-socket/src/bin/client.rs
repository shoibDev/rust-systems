use std::{io::Write, io::stdin, net::TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server!");

    // its a pointer to an array??
    let data = b"Hello"; // b tells it to make ti an array of bits.. in ASCII
    // let _bytes_written = stream.write_all(data);
    // stream.flush().unwrap();
    // println!("Disconnected");

    let mut message: String = String::new();

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
    }
}
