use crate::Response;
use std::{error, fmt};

pub trait Responder {
    fn to_response(self) -> Response;
}

impl Responder for Response {
    fn to_response(self) -> Response {
        self
    }
}

impl Responder for &str {
    fn to_response(self) -> Response {
        Response::from(self.to_string())
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

impl<T: Responder, E: error::Error> Responder for Result<T, E> {
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
            None => Response::from(404),
            Some(s) => s.to_response(),
        }
    }
}
