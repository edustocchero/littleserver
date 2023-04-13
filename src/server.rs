use std::io::Error;
use std::net::{TcpListener, ToSocketAddrs};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new<A: ToSocketAddrs>(address: A) -> Result<Server, Error> {
        let result: Result<TcpListener, _> = TcpListener::bind(address);

        result.map(|listener| Server { listener })
    }
}
