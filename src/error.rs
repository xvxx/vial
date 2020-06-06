use std::{error::Error as ErrorTrait, fmt, io};

/// Possible Vial errors.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Unknown HTTP method. Methods should be "ALL-CAPS".
    UnknownHTTPMethod(String),
    /// Failed to parse HTTP Version.
    ParseVersion,
    /// Expected \r\n but didn't find it.
    ExpectedCRLF,
    /// Failed to parse HTTP header name.
    ParseHeaderName,
    /// Failed to parse HTTP header value.
    ParseHeaderValue,
    /// Failed to parse HTTP request.
    ParseError,
    /// io::Error
    IO(String),
    /// Unknown error.
    Other(String),
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnknownHTTPMethod(reason) => &reason,
            Error::ParseVersion => "Error Parsing HTTP Version",
            Error::ExpectedCRLF => "Expected CRLF in HTTP Request",
            Error::ParseHeaderName => "Error Parsing HTTP Header name",
            Error::ParseHeaderValue => "Error Parsing HTTP Header value",
            Error::ParseError => "Error Parsing HTTP Request",
            Error::IO(reason) => &reason,
            Error::Other(reason) => &reason,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vial::Error: {}", self)
    }
}
