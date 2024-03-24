use anyhow::Error;
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

enum Status {
    200,
    403,
    404,
    500,
}

impl TCPServer {
    pub fn new(host: &str, port: &str) -> Self {
        let host = host.to_string();
        let port = port.to_string();
        TCPServer { host, port }
    }
    pub fn start(self) -> Result<(), Error> {
        let socket_addr = format!("{}:{}", self.host, self.port);
        println!("Attempting to bind to {}", &socket_addr);
        let listener = TcpListener::bind(socket_addr);
        let (mut stream, addr) = listener.unwrap().accept()?;
        println!("Acceping connections on {}", &addr);
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        dbg!(buffer);
        println!("Read {} Bytes", bytes_read);
        let response = TCPServer::handle_request(&buffer);
        let send_res = stream.write(&response)?;
        println!("Sent {} Bytes Back", send_res);
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }

    fn handle_request(_data: &[u8]) -> Vec<u8> {
        let response_line = "HTTP/1.1 200 OK \r\n".as_bytes();
        let headers = [
            "Server: Collin's based http server\r\n".as_bytes(),
            "Content-Type: text/html\r\n".as_bytes(),
        ]
        .concat();
        let blank_line = "\r\n".as_bytes();
        let response_body = "<html><body><h1>Message Recieved</h1></body></html>".as_bytes();
        [response_line, &headers, blank_line, response_body].concat()
    }

    fn create_response_line(status: i32){
        let reason = match status =>
        {
        (Status::200) => "Ok",
        (Status::500) => "Internal Server Error",
        (Status::404) => "Not Found",
    }
        let response_line = format!("HTTP/1.1 {} {} \r\n",reason, status).as_bytes();

    }
}

fn main() {
    let server = TCPServer::new(HOST, PORT);
    server.start().unwrap();
}
