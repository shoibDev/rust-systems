use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;

    println!("Connected to server!");

    Ok(())
}
