use crate::Error;

/// Method is just an enum representing the HTTP methods Vial
/// supports. Which is not all of them.
#[derive(PartialEq, Eq, Hash, Debug)]
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
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "PATCH" => Self::PATCH,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            _ => return Err(Error::UnknownHTTPMethod(s.into())),
        })
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(Self::GET)
    }
}
