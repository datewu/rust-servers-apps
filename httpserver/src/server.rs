use super::router::Router;
use http::httprequest::HTTPRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self { socket_addr }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on{}", self.socket_addr);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buf = [0; 90];
            stream.read(&mut read_buf).unwrap();
            let req: HTTPRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
