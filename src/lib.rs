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

/// Assets bundled into the binary in release mode.
pub static mut BUNDLED_ASSETS: Option<std::collections::HashMap<String, &'static [u8]>> = None;

/// Date and time this program was compiled.
pub const BUILD_DATE: &str = env!("BUILD_DATE");

#[cfg(feature = "stateful")]
use state;

#[cfg(feature = "stateful")]
static mut STORAGE: Option<state::Container> = None;

#[cfg(feature = "stateful")]
pub fn storage_init() {
    unsafe {
        STORAGE = Some(state::Container::new());
    }
}

#[cfg(feature = "stateful")]
pub fn storage_get<T: Send + Sync + 'static>() -> &'static T {
    unsafe { STORAGE.as_ref().unwrap().get::<T>() }
}

#[cfg(feature = "stateful")]
pub fn storage_set<T: Send + Sync + 'static>(o: T) {
    unsafe {
        STORAGE.as_ref().unwrap().set(o);
    }
}

#[cfg(feature = "stateful")]
pub struct State<T: Send + Sync + 'static> {
    request: Request,
    phantom: std::marker::PhantomData<T>,
}

#[cfg(feature = "stateful")]
impl<T: Send + Sync + 'static> std::ops::Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        storage_get::<T>()
    }
}

#[cfg(feature = "stateful")]
impl<T: Send + Sync + 'static> From<Request> for State<T> {
    fn from(request: Request) -> State<T> {
        State {
            request,
            phantom: std::marker::PhantomData,
        }
    }
}
