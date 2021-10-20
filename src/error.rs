use std::{error::Error as ErrorTrait, fmt, io};

/// Possible Vial errors.
#[derive(Debug)]
pub enum Error {
    /// Can't find an asset.
    AssetNotFound(String),
    /// Client closed the HTTP connection.
    ConnectionClosed,
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
    IO(io::Error),
    /// Unknown error.
    Other(String),
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        Self::new(io::ErrorKind::Other, err.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl ErrorTrait for Error {
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            Error::IO(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::UnknownHTTPMethod(reason) => reason,
                Error::ConnectionClosed => "Connection Closed By Client",
                Error::ParseVersion => "Error Parsing HTTP Version",
                Error::ExpectedCRLF => "Expected CRLF in HTTP Request",
                Error::ParseHeaderName => "Error Parsing HTTP Header name",
                Error::ParseHeaderValue => "Error Parsing HTTP Header value",
                Error::ParseError => "Error Parsing HTTP Request",
                Error::AssetNotFound(..) => "Can't Find Asset",
                Error::IO(..) => "io::Error While Parsing HTTP Request",
                Error::Other(reason) => reason,
            }
        )
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::{AssetNotFound, ConnectionClosed, ExpectedCRLF, IO, Other, ParseError, ParseHeaderName, ParseHeaderValue, ParseVersion, UnknownHTTPMethod};
        match self {
            IO(_) => false,
            AssetNotFound(s) => match other {
                AssetNotFound(o) => s == o,
                _ => false,
            },
            UnknownHTTPMethod(s) => match other {
                UnknownHTTPMethod(o) => s == o,
                _ => false,
            },
            Other(s) => match other {
                Other(o) => s == o,
                _ => false,
            },
            ConnectionClosed => matches!(other, ConnectionClosed),
            ParseVersion => matches!(other, ParseVersion),
            ExpectedCRLF => matches!(other, ExpectedCRLF),
            ParseHeaderName => matches!(other, ParseHeaderName),
            ParseHeaderValue => matches!(other, ParseHeaderValue),
            ParseError => matches!(other, ParseError),
        }
    }
}
