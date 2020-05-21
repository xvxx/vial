#![allow(unused)]
use vial::{vial, Responder};

vial! {
    GET "/hello/:name" => |req| {
        let name = req.arg("name").unwrap_or("");
        format!("Hello, {}", name)
    };
}

fn main() {
    vial::run!();
}
