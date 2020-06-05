use crate::Error;

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

impl std::str::FromStr for Method {
    type Err = Error;

    /// Converts an "ALL-CAPS" HTTP method into our enum.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            _ => return Err(Error::UnknownHTTPMethod(s.into())),
        })
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        s.parse().unwrap_or(Method::GET)
    }
}
