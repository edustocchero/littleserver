mod server;

use crate::server::Server;

fn main() {
    println!("Hello, world!");

    let server_result: Result<Server, _> = Server::new("127.0.0.1:80");

    let server = match server_result {
        Ok(server) => {
            println!("Server created successfully!");
            server
        },
        Err(e) => panic!("Could not create server: {:?}", e)
    };

    println!("Server state: {:?}", server);
    server.listen();
}
