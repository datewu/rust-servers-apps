use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Runing on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buff = [0; 1024];
        stream.read(&mut buff).unwrap();
        stream.write(&mut buff).unwrap();
    }
}
