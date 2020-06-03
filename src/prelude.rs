//! The prelude pulls in all the main types, traits, and macros:
//!
//! - [Request](struct.Request.html)
//! - [Response](struct.Response.html)
//! - [Responder](trait.Responder.html)
//! - [Method](enum.Method.html)
//! - [Router](struct.Router.html)
//! - [State](struct.State.html) (with `--features state`)
//! - [run!](macro.run.html)
//! - [routes!](macro.routes.html)

pub use crate::{
    asset, method::Method, request::Request, responder::Responder, response::Response,
    router::Router, routes, run,
};

#[cfg(feature = "state")]
pub use crate::{storage::State, use_state};

#[cfg(features = "horror")]
pub use features::horrorshow::{box_html, html, owned_html};
