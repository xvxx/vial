//! The prelude pulls in all the main types, traits, and macros:
//!
//! - [`Request`](struct.Request.html)
//! - [`Response`](struct.Response.html)
//! - [`Responder`](trait.Responder.html)
//! - [`Method`](enum.Method.html)
//! - [`Router`](struct.Router.html)
//! - [`run!`](macro.run.html)
//! - [`routes!`](macro.routes.html)
//! - [`asset_dir!`](macro.asset_dir.html)
//!
//! With the `state` feature enabled, it also pulls in:
//! - [`use_state!`](macro.use_state.html)

pub use crate::{
    asset, asset_dir, method::Method, request::Request, responder::Responder, response::Response,
    router::Router, routes, run,
};

#[cfg(feature = "state")]
pub use crate::use_state;

#[cfg(features = "horror")]
pub use features::horrorshow::{box_html, html, owned_html};
