#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    PATCH,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl Method {
    pub fn from(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            _ => Method::GET,
        }
    }
}
