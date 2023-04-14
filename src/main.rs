mod server;

use std::fs;
use std::sync::{Arc};
use std::net::TcpListener;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut resource_map: HashMap<String, String> = HashMap::new();
    let resource_path = fs::read_dir("./resources").unwrap();

    resource_path.for_each(|resource| {
        let path = resource.unwrap().path();
        if path.is_file() {
            let contents = fs::read_to_string(path.clone()).unwrap();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            resource_map.insert(format!("/{name}"), contents);
        }
    });

    let listener = TcpListener::bind("localhost:80").unwrap();

    let resources = Arc::new(resource_map);

    server::listen(&listener, resources);
}
