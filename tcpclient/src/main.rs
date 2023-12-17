use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello, world!".as_bytes()).unwrap();
    let mut buff = [0; 20];
    stream.read(&mut buff).unwrap();
    println!(
        "Got response from server: {:?}",
        str::from_utf8(&buff).unwrap()
    )
}
