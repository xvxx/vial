#![allow(unused)]

#[macro_use]
mod macros;
pub mod asset;
mod bundler;
pub mod features;
mod method;
pub mod prelude;
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
    bundler::bundle_assets, method::Method, request::Request, responder::Responder,
    response::Response, router::Router, server::run,
};
pub type Result<T> = std::result::Result<T, std::io::Error>;

/// Directory where assets are stored, if any.
pub static mut ASSET_DIR: Option<&'static str> = None;

