use std::net::{TcpListener};
use std::io::{Write, Result, Read};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12346")?;
    println!("Bound!");
    let mut buffer: [u8; 1024] = [0;1024];
    for stream in listener.incoming() {
        println!("Client connected, writing");
        let mut stream = stream?;
        println!("Converted stream var");
        stream.read(&mut buffer)?;
        println!("Received {}", String::from_utf8_lossy(&buffer));
        stream.write(&buffer)?;
    }
    Ok(())
}
