// Copyright 2020 The Vial Authors
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # ~ vial: a micro micro-framework ~
//!
//! **Vial** is a small web "framework" for making small web sites.
//!
//! It only includes a few basics:
//!
//! - **[routes!](macro.routes.html)**: Macro for mapping URLs to
//!   handlers.
//! - **[Request](struct.Request.html)**: Information about the
//!   current request, such as form data or URL path segments.
//! - **[Response](struct.Response.html)**: Response to deliver to the
//!   client.
//! - **[Responder](trait.Responder.html)**: Trait to convert your
//!   types or a few built-ins like `String` into a `Response`.
//! - **[asset](module.Asset.html)**: Serving of static files and
//!   support for bundling into the release binary.
//!
//! Everything else... well, that's up to you.
//!
//! The goal is an as-few-as-possible-dependencies web library you can
//! use to test out an idea quickly or get a personal project _rolling_.
//! Single file, server side apps? You bet! Fast compilation? Yes please!
//! _À la carte_ dependencies? Now you're talkin'!
//!
//! It's sort of like a picnic where the playlist is all 90s music and you
//! have to bring your own beverages. And food.
//!
//! ----
//!
//! **Status:** Vial is currently in early development. It is being
//! developed alongside [deadwiki], but that is _strictly_ for personal
//! use. Proceed with caution.
//!
//! ---
//!
//! To get started, just add `vial` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vial = "*"
//! ```
//!
//! There are a handful of `--features` that you can enable, most of
//! which add additional dependencies that need to be included:
//!
//! ```toml
//! [dependencies]
//! vial = { version = "*", features = ["state", "cookies"] }
//! ```
//!
//! This list is a work in progress:
//!
//! - [x] **state**: Global state: handlers take `State<T>`
//! - [x] **horror**: Small & fast macro-based HTML builder, via [horrowshow].
//! - [ ] **cookies**: Cookie monster!
//! - [ ] **sessions**: Session support
//! - [ ] **multipart**: Multipart form data (file uploads)
//! - [ ] **log**: Access logging
//! - [ ] **json**: `to_json` and `from_json` powers, via Serde.
//!
//! ## ~ hello world ~
//!
//! As is tradition:
//!
//! ```rust
//! vial::routes! {
//!     GET "/" => |_| "Hello, world!";
//! }
//!
//! fn main() {
//!     vial::run!().unwrap();
//! }
//! ```
//!
//! For a bit more sanity, you can route to functions directly:
//!
//! ```rust
//! use vial::prelude::*;
//!
//! routes! {
//!     GET "/echo" => echo;
//!     POST "/echo" => post;
//! }
//!
//! fn echo(_: Request) -> &'static str {
//!     "<form method='POST'>
//!         <input type='text' name='echo'/>
//!         <input type='submit'/>
//!     </form>"
//! }
//!
//! fn post(req: Request) -> String {
//!     format!(
//!         "<h1>{}</h1>",
//!         req.form("echo").unwrap_or("You didn't say anything!")
//!     )
//! }
//!
//! fn main() {
//!     vial::run!().unwrap();
//! }
//! ```
//!
//! To _really_ break the mold, you can split your site into different
//! modules:
//!
//! ```rust
//! use vial;
//!
//! mod wiki;
//! mod blog;
//!
//! mod index {
//!     use vial::prelude::*;
//!     routes! {
//!         GET "/" => |_| Response::from_file("index.html")
//!     }
//! }
//!
//! fn main() {
//!     // The order matters here - if `wiki` and `blog` both define "/",
//!     // the `mod index` version will match first and get run.
//!     vial::run!(index, wiki, blog);
//! }
//! ```
//!
//! But hey, who wants to putz around with HTML when you can be writing
//! **Rust**? Enable the `horror` feature and you're on your way:
//!
//! ```rust
//! use vial::prelude::*;
//!
//! routes! {
//!     GET "/" => |_| html! {
//!         p {
//!             : "You're looking for this: ";
//!             a(href="/echo") { : "echo" }
//!         }
//!     };
//!     GET "/echo" => echo;
//!     POST "/echo" => post;
//! }
//!
//! fn echo(_: Request) -> impl Responder {
//!     html! {
//!         form(method="POST") {
//!             p {
//!             : "Type something: ";
//!                 input(type="text", name="echo");
//!                 input(type="submit");
//!             }
//!         }
//!     }
//! }
//!
//! fn post(req: Request) -> impl Responder {
//!     owned_html! {
//!         h1: req.form("echo")
//!             .unwrap_or("You didn't say anything!");
//!     }
//! }
//!
//! fn main() {
//!     vial::run!().unwrap();
//! }
//! ```
//!
//! ## ~ hot reloading ~
//!
//! Install [cargo-watch]:
//!
//!     $ cargo install cargo-watch
//!     $ cargo watch -x 'run --example hello_world'
//!
//! ## ~ performance ~
//!
//! We want to keep **Vial** snappy, but this is not a production web
//! server that's competing in any shootouts. Our bigger priority is
//! keeping the base dependency count and compilation time low.
//!
//! ## ~ when to use ~
//!
//! Probably never, or only ever to quickly test an idea. Certainly
//! not for personal wikis or small hobby projects, unless you
//! insisted.
//!

#![warn(missing_docs)]

#[macro_use]
mod macros;
pub mod asset;
mod bundler;
mod cache;
pub mod features;
mod method;
pub mod prelude;
mod request;
mod responder;
mod response;
mod router;
mod server;
mod util;

#[cfg(feature = "state")]
pub mod storage;
#[cfg(feature = "state")]
pub use storage::State;

#[cfg(features = "horror")]
#[macro_use]
extern crate horrorshow;

#[cfg(features = "horror")]
pub use features::horrorshow::{box_html, html, owned_html};

pub use {
    bundler::bundle_assets, cache::TypeCache, method::Method, request::Request,
    responder::Responder, response::Response, router::Router, server::run,
};

pub type Result<T> = std::result::Result<T, std::io::Error>;

/// Directory where assets are stored, if any.
pub static mut ASSET_DIR: Option<&'static str> = None;

/// Assets bundled into the binary in release mode.
pub static mut BUNDLED_ASSETS: Option<std::collections::HashMap<String, &'static [u8]>> = None;

/// Date and time this program was compiled.
pub const BUILD_DATE: &str = env!("BUILD_DATE");
