use std::{io::Write, net::TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server!");

    // its a pointer to an array??
    let data = b"Hello"; // b tells it to make ti an array of bits.. in ASCII
    let _bytes_written = stream.write_all(data);
    stream.flush().unwrap();
    println!("Disconnected")
}
