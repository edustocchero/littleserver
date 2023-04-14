use std::{thread};
use std::sync::{Arc};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

const NOT_FOUND_RESPONSE: &[u8; 63] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 17\r\n\r\nContent not found";
const INTERNAL_SERVER_ERROR_RESPONSE: &[u8; 38] = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";

pub fn listen(listener: &TcpListener, resources: Arc<HashMap<String, String>>) {
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                let resources = Arc::clone(&resources);
                thread::spawn(move || handle_stream(&mut stream, resources));
            }
            Err(e) => panic!("Could not accept: {:?}", e)
        }
    }
}

fn handle_stream(stream: &mut TcpStream, resources: Arc<HashMap<String, String>>) {
    let mut buffer: [u8; 1024] = [0; 1024];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(len) => len,
        Err(e) => panic!("Could not read to buffer: {:?}", e)
    };

    if !(bytes_read > 0) {
        write_internal_server_error(stream);
        return
    }

    let mut reader = BufReader::new(&buffer[..bytes_read]);
    let mut first_line: String = String::new();
    reader.read_line(&mut first_line).expect("Could read the first line");
    let request: Request = Request::new(first_line);

    if resources.contains_key(&request.path) {
        let content = resources.get(&request.path).unwrap();
        write_html(stream, content);
    } else {
        write_not_found(stream);
    }
    stream.shutdown(Shutdown::Both).unwrap();
}

fn write_html(stream: &mut TcpStream, content: &String) {
    let len = content.len();
    let fmt = format!("HTTP/1.1 200 OK\r\nContent-Length: {len}\r\nContent-Type: text/html\r\n\r\n{content}");
    stream.write_all(fmt.as_bytes()).unwrap();
}

fn write_not_found(stream: &mut TcpStream) {
    stream.write_all(NOT_FOUND_RESPONSE).unwrap();
}

fn write_internal_server_error(stream: &mut TcpStream) {
    stream.write_all(INTERNAL_SERVER_ERROR_RESPONSE).unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Request {
    method: Method,
    path: String,
}

impl Request {
    fn new(src: String) -> Request {
        let mut iter = src.split_whitespace();

        let method = Method::from_string(iter.next().expect("Has method"));
        let path = iter.next().expect("Has path").to_string();

        Request { method, path }
    }
}
