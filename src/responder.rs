use crate::Response;

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
