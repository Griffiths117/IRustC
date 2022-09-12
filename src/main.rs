use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Result, Read};
fn echo(stream: &mut TcpStream, buf: &mut [u8]) -> Result<()> {
    stream.read(buf)?;
    stream.write(buf)?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12346")?;
    println!("Bound!");
    let mut buffer: [u8; 1024] = [0;1024];
    for stream in listener.incoming() {
        let res = echo(&mut stream?, &mut buffer);
        if res.is_err(){
            eprintln!("Error {}", res.unwrap_err());
            continue;
        }
        println!("Received {}", String::from_utf8_lossy(&buffer));
        if buffer == [0; 1024] {
            break;
        }
        buffer.fill(0);
    }
    println!("EXITING");
    Ok(())
}
