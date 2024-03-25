use anyhow::Error;
use std::fmt;
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

pub enum Status {
    OK = 200,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
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
        let response_line = Self::create_response_line(Status::OK);
        let headers = Self::create_headers(&[
            "Server: Collin's based http server\r\n",
            "Content-Type: text/html\r\n",
        ]);
        let blank_line = "\r\n".as_bytes();
        let response_body = "<html><body><h1>Message Recieved</h1></body></html>".as_bytes();
        [&response_line, &headers, blank_line, response_body].concat()
    }

    fn create_headers(headers: &[&str]) -> Vec<u8> {
        let mut binary_headers: Vec<u8> = vec![];
        for header in headers {
            binary_headers.extend(header.as_bytes());
        }
        binary_headers
    }

    fn create_response_line(status: Status) -> Vec<u8> {
        let reason = match &status {
            Status::OK => "Ok",
            Status::NotFound => "Not Found",
            Status::Forbidden => "Forbidden",
            Status::InternalServerError => "Internal Server Error",
        };
        format!("HTTP/1.1 {} {} \r\n", reason, status)
            .as_bytes()
            .to_vec()
    }
}

struct HTTPRequest {
    method: String,
    uri: String,
    http_version: i32,
}

impl HTTPRequest {
    fn parse(request: &[&u8]) -> Self {
        todo!();
    }
}

fn main() {
    let server = TCPServer::new(HOST, PORT);
    server.start().unwrap();
}
