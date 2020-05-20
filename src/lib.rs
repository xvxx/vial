#![allow(unused)]

#[macro_use]
mod macros;
mod method;
mod request;
mod response;
mod server;
mod util;

pub use {request::Request, response::Response, server::run};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
