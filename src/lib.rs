#![allow(unused)]

#[macro_use]
mod macros;
mod asset;
mod features;
mod method;
mod request;
mod response;
mod router;
mod server;
mod util;

pub use {method::Method, request::Request, response::Response, router::Router, server::run};
pub type Result<T> = std::result::Result<T, std::io::Error>;
