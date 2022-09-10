use std::io;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:12345")?;
    for stream in listener.incoming() {
        stream?.write("Hello, World!");
    }

}
