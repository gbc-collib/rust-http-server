use std::{
    io::{Read, Write},
    net::TcpListener,
};
const HOST: &'static str = "127.0.0.1";
const PORT: &'static str = "8080";

pub struct TCPServer {
    pub host: String,
    pub port: String,
}

impl TCPServer {
    pub fn new(host: &str, port: &str) -> Self {
        let host = host.to_string();
        let port = port.to_string();
        TCPServer { host, port }
    }
    pub fn start(self) {
        let socket_addr = format!("{}:{}", self.host, self.port);
        println!("Attempting to bind to {}", &socket_addr);
        let listener = TcpListener::bind(socket_addr);
        let (mut stream, addr) = listener.unwrap().accept().unwrap();
        println!("Acceping connections on {}", &addr);
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        dbg!(buffer);
        println!("Read {} Bytes", bytes_read);
        let send_res = stream.write(&buffer).unwrap();
        println!("Sent {} Bytes Back", send_res);
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

fn main() {
    let server = TCPServer::new(HOST, PORT);
    server.start();
}
