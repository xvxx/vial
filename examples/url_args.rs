#![allow(unused)]
use vial::{vial, Responder};

vial! {
    GET "/hello/:id/:name" => |req| {
        let name = req.arg("name").unwrap_or("");
        format!("Hello, {}. You're ID #{}", name, req.arg("id").unwrap_or("0"))
    };
}

fn main() {
    vial::run!();
}
