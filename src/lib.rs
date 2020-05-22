#![allow(unused)]

#[macro_use]
mod macros;
mod asset;
mod features;
mod method;
mod request;
mod responder;
mod response;
mod router;
mod server;
mod util;

#[cfg(features = "horror")]
#[macro_use]
extern crate horrorshow;

#[cfg(features = "horror")]
pub use features::horrorshow::{box_html, html, owned_html};

pub use {
    method::Method, request::Request, responder::Responder, response::Response, router::Router,
    server::run,
};
pub type Result<T> = std::result::Result<T, std::io::Error>;
