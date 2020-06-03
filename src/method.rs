/// Method is just an enum representing the HTTP methods Vial
/// supports. Which is not all of them.
#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    /// HTTP GET
    GET,
    /// HTTP HEAD
    HEAD,
    /// HTTP POST
    POST,
    /// HTTP PUT
    PUT,
    /// HTTP DELETE
    DELETE,
    /// HTTP PATCH
    PATCH,
    /// HTTP OPTIONS
    OPTIONS,
    /// HTTP TRACE
    TRACE,
}

impl Method {
    /// Converts an ALL-CAPS HTTP method into our enum.
    pub fn from_str(s: &str) -> Method {
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

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        Method::from_str(s)
    }
}
