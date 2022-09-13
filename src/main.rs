use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Result, Read};
fn echo(stream: &mut TcpStream) -> Result<String> {
    let mut buffer: [u8; 1024] = [0;1024];
    let buflen = stream.read(&mut buffer)?;
    // Trim trailing NULL bytes and store as vector
    let buffer = buffer[..buflen].to_vec();

    stream.write(&buffer)?;
    let str = String::from_utf8_lossy(&buffer);
    stream.shutdown(Shutdown::Both)?;
    Ok(str.to_string())
}
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    println!("Bound!");
    for stream in listener.incoming() {
        let res = echo(&mut stream?);
        if res.is_err(){
            eprintln!("Error {}", res.unwrap_err());
            continue;
        }
        let msg = res.unwrap();
        println!("Received: {} Len: {}", msg, msg.len());
        if msg == "STOP" {
            break;
        }
    }
    println!("EXITING");
    Ok(())
}
