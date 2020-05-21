#![allow(unused)]

#[macro_use]
mod macros;
mod features;
mod method;
mod request;
mod response;
mod server;
mod util;

pub use {request::Request, response::Response, server::run};
pub type Result<T> = std::result::Result<T, std::io::Error>;
