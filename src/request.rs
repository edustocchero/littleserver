use std::collections::HashMap;
use std::io::{BufRead, BufReader, Lines};
use std::str::SplitWhitespace;

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    OTHER,
}

impl Method {
    fn from_str(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::OTHER
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    #[deprecated]
    pub fn new(src: String) -> Request {
        let mut iter = src.split_whitespace();

        let method: Method = Method::from_str(iter.next().expect("Has method"));
        let path: String = iter.next().expect("Has path").to_string();

        Request { method, path, headers: Default::default() }
    }

    pub fn from_bytes(bytes: &[u8]) -> Request {
        let mut reader: BufReader<&[u8]> = BufReader::new(bytes);

        let mut first_line: String = String::new();
        reader.read_line(&mut first_line).expect("Could read the first line");

        let mut iter: SplitWhitespace = first_line.split_whitespace();

        let str_method: &str = iter.next().expect("Has method");
        let path: String = iter.next().expect("Has path").to_string();
        let _version = iter.next().expect("Has version");

        let method: Method = Method::from_str(str_method);

        let mut headers: HashMap<String, String> = HashMap::new();

        let lines: Lines<BufReader<&[u8]>> = reader.lines();
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }
                    let key_val: Option<(&str, &str)> = line.split_once(": ");
                    match key_val {
                        None => {}
                        Some(key_val) => {
                            let key: String = key_val.0.to_string();
                            let val: String = key_val.1.to_string();
                            headers.insert(key, val);
                        }
                    }
                }
                Err(e) => panic!("Error reading line: {:?}", e)
            }
        }

        Request {
            method,
            path,
            headers,
        }
    }
}
