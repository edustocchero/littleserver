mod server;
mod request;

use crate::server::Server;

fn main() {
    println!("Hello, world!");

    let mut server = Server::new("127.0.0.1:8080", "./resources");
    server.listen();
}
