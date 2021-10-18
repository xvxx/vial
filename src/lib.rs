#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    clippy::all,
    // missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]


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
//! - **[asset](asset/index.html)**: Serving of static files and
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
//! To learn more, keep reading or visit one of these links:
//!
//! - [Manual](https://vial.rs/)
//! - [Source Code](https://github.com/xvxx/vial)
//! - [Bug Tracker](https://github.com/xvxx/vial/issues)
//! - [Crate](https://crates.io/crates/vial)
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
//! vial = "0.1"
//! ```
//!
//! There are a handful of `--features` that you can enable, most of
//! which add additional dependencies that need to be included:
//!
//! ```toml
//! [dependencies]
//! vial = { version = "*", features = ["horror", "cookies"] }
//! ```
//!
//! This list is a work in progress:
//!
//! - [x] **horror**: Small & fast macro-based HTML builder, via [horrowshow].
//! - [x] **json_serde**: `Request::json` and `Response::with_json` powers,
//!       via Serde.
//! - [x] **cookies**: Cookie monster!
//! - [ ] **sessions**: Session support
//! - [ ] **multipart**: Multipart form data (file uploads)
//! - [ ] **log**: Access logging
//!
//! ## ~ hello world ~
//!
//! As is tradition:
//!
//! ```no_run
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
//! ```no_run
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
//! ```no_run
//! mod wiki {
//!     vial::routes! {
//!         GET "/wiki" => |_| Response::from_file("wiki.html");
//!         // etc...
//!     }
//! }
//!
//! mod blog {
//!     vial::routes! {
//!         GET "/blog" => |_| Response::from_file("wiki.html");
//!         // etc...
//!     }
//! }
//!
//! mod index {
//!     use vial::prelude::*;
//!     routes! {
//!         GET "/" => |_| Response::from_file("index.html");
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
//! ```ignore
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
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::large_enum_variant)]

#[macro_use]
mod macros;
pub mod asset;

mod cache;
mod error;
mod method;
pub mod prelude;
mod request;
mod responder;
mod response;
mod router;
mod server;

// used in tests
#[doc(hidden)]
pub mod bundler;
#[doc(hidden)]
pub mod http_parser;
#[doc(hidden)]
pub mod util;

#[doc(hidden)]
pub mod storage;

#[cfg(feature = "horror")]
#[doc(hidden)]
pub mod horrorshow;

pub use {
    bundler::bundle_assets, cache::TypeCache, error::Error, method::Method, request::Request,
    responder::Responder, response::Response, router::Router, server::run,
};

/// Convenience Result that returns `vial::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Directory where assets are stored, if any.
pub static mut ASSET_DIR: Option<String> = None;

/// Assets bundled into the binary in release mode.
pub static mut BUNDLED_ASSETS: Option<std::collections::HashMap<String, &'static [u8]>> = None;

/// Date and time this program was compiled.
pub const BUILD_DATE: &str = env!("BUILD_DATE");

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
