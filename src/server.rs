use std::{fs, thread, time};
use std::sync::Arc;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use crate::request::Request;

static NOT_FOUND_RESPONSE: &[u8; 63] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 17\r\n\r\nContent not found";
static INTERNAL_SERVER_ERROR_RESPONSE: &[u8; 38] = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";

pub struct Server {
    listener: TcpListener,
    mappings: Arc<HashMap<String, String>>,
}

impl Server {
    pub fn new<A: ToSocketAddrs>(addr: A, resource_path: &str) -> Server {
        let listener = match TcpListener::bind(addr) {
            Ok(tcp_listener) => tcp_listener,
            Err(e) => panic!("Could not bind: {:?}", e)
        };

        let mut resource_map: HashMap<String, String> = HashMap::new();
        let resources = match fs::read_dir(resource_path) {
            Ok(dir) => dir,
            Err(e) => panic!("Could not read dir: {:?}", e)
        };

        resources.for_each(|resource| {
            let path = resource.unwrap().path();
            if path.is_file() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let contents = fs::read_to_string(path).unwrap();
                println!("Found: {:?}", &name);
                println!("{:?} size: {:?}", &name, contents.len());
                if name.eq("index.html") {
                    resource_map.insert("/".to_string(), contents.clone());
                }
                resource_map.insert(format!("/{name}"), contents);
            }
        });

        let mappings: Arc<HashMap<String, String>> = Arc::new(resource_map);

        Server { listener, mappings }
    }

    pub fn listen(&mut self) {
        println!("Listening on {:?}", self.listener.local_addr().unwrap());
        for result in self.listener.incoming() {
            match result {
                Ok(mut stream) => {
                    println!("New connection: {addr}", addr = stream.peer_addr().unwrap());
                    let resources = Arc::clone(&self.mappings);
                    thread::spawn(move || handle_stream(&mut stream, &resources));
                }
                Err(e) => panic!("Could not listen: {:?}", e)
            };
        }
    }
}

fn handle_stream(stream: &mut TcpStream, resources: &Arc<HashMap<String, String>>) {
    let mut buffer: [u8; 4096] = [0; 4096];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(len) => len,
        Err(e) => panic!("Could not read to buffer: {:?}", e)
    };

    if !(bytes_read > 0) {
        return;
    }

    let request = Request::from_bytes(&buffer[..bytes_read]);

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
