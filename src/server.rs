use std::fs;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new<A: ToSocketAddrs>(address: A) -> Result<Server, Error> {
        let result: Result<TcpListener, _> = TcpListener::bind(address);

        result.map(|listener| Server { listener })
    }

    pub fn listen(&self) {
        loop {
            let (mut stream, _) = match self.listener.accept() {
                Err(e) => panic!("Could not accept: {:?}", e),
                Ok(result) => result
            };

            let mut buffer: [u8; 256] = [0; 256];

            let bytes_read = match stream.read(&mut buffer) {
                Ok(len) => len,
                Err(e) => panic!("Could not read to buffer: {:?}", e)
            };

            let mut reader = BufReader::new(&buffer[..bytes_read]);
            let mut first_line: String = String::new();
            reader.read_line(&mut first_line).expect("Could read the first line");

            let request: Request = Request::new(&first_line);
            println!("Request state: {:?}", request);

            Server::respond(request, &mut stream);
        }
    }

    fn respond(request: Request, stream: &mut TcpStream) {
        match request.method {
            Method::GET => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n"),
            Method::POST => stream.write_all(b"HTTP/1.1 204 No Content\r\n\r\n"),
            Method::OTHER => stream.write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n"),
        }.expect("Processes the request");
    }
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    OTHER,
}

impl Method {
    fn from_string(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::OTHER
        }
    }
}

#[derive(Debug)]
struct Request<'a> {
    method: Method,
    path: &'a str,
}

impl<'a> Request<'a> {
    fn new(src: &'a String) -> Request<'a> {
        let mut iter = src.split_whitespace();

        let method = Method::from_string(iter.next().expect("Has method"));
        let path = iter.next().expect("Has path");

        Request { method, path }
    }
}
