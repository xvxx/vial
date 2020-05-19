#![allow(unused)]

use std::collections::HashMap;
use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

#[macro_use]
extern crate vial;
use vial::{vial, Request, Response};

vial! {
    GET "/blahblah" => blahblah;

    POST "/blah" => |_| "Dingo!".into();

    GET "/hello/world" => |_| "Hiya".into();

    GET "/info" => |req| {
        format!("<h1>Request Information:</h1><pre>{:?}</p>", req).into()
    };

    GET "/" => |_| {
        // sinatra-like, i guess
        "Cool".into()
    };
}

fn blahblah(req: Request) -> Response {
    let default = "20".to_string();
    "blah "
        .repeat(
            req.param("times")
                .unwrap_or_else(|| &default)
                .parse()
                .unwrap(),
        )
        .into()
}

fn main() {
    vial::run("0.0.0.0:7667");
}
