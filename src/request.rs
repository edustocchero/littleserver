#[derive(Debug, Clone)]
pub enum Method {
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
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
}

impl Request {
    pub fn new(src: String) -> Request {
        let mut iter = src.split_whitespace();

        let method = Method::from_string(iter.next().expect("Has method"));
        let path = iter.next().expect("Has path").to_string();

        Request { method, path }
    }
}
