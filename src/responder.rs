use crate::Response;

/// The `Responder` trait converts your custom types or a few basic
/// Rust types into a [Response](struct.Response.html) that gets hand
/// delivered to the HTTP client in a timley fashion, barring any
/// weather delays.
pub trait Responder {
    /// Consume this object and return a
    /// [Response](struct.Response.html) representing it.
    fn to_response(self) -> Response;
}

impl Responder for Response {
    fn to_response(self) -> Response {
        self
    }
}

impl Responder for &str {
    fn to_response(self) -> Response {
        Response::from(self)
    }
}

impl Responder for String {
    fn to_response(self) -> Response {
        Response::from(self)
    }
}

impl Responder for usize {
    fn to_response(self) -> Response {
        Response::from(self)
    }
}

impl<T: Responder, E: std::error::Error> Responder for Result<T, E> {
    fn to_response(self) -> Response {
        match self {
            Err(e) => Response::from_error(e),
            Ok(s) => s.to_response(),
        }
    }
}

impl<T: Responder> Responder for Option<T> {
    fn to_response(self) -> Response {
        match self {
            None => Response::from_code(404),
            Some(s) => s.to_response(),
        }
    }
}

impl Responder for crate::Error {
    fn to_response(self) -> Response {
        Response::from_error(self)
    }
}

impl Responder for () {
    fn to_response(self) -> Response {
        Response::default()
    }
}
