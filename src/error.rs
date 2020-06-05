use std::{error::Error as ErrorTrait, fmt};

/// Possible Vial errors.
#[derive(Debug)]
pub enum Error {
    /// Unknown HTTP method. Methods should be "ALL-CAPS".
    UnknownHTTPMethod(String),
    /// Unknown error.
    Other(String),
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnknownHTTPMethod(reason) => &reason,
            Error::Other(reason) => &reason,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vial::Error: {}", self)
    }
}
