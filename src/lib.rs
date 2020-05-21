#![allow(unused)]

#[macro_use]
mod macros;
#[cfg(feature = "horror")]
mod horrorshow;
mod method;
mod request;
mod response;
mod server;
mod util;

pub use {request::Request, response::Response, server::run};
pub type Result<T> = std::result::Result<T, std::io::Error>;
