use std::io::{BufRead, BufReader, Error, Read};
use std::net::{TcpListener, ToSocketAddrs};

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

            println!("First line: {:?}", first_line);
        }
    }
}
