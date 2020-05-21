#[macro_use]
extern crate horrorshow;
use vial::{vial, Request, Responder, Response};

vial! {
    GET "/" => |_| html! {
        p {
            : "You're looking for this: ";
            a(href="/echo") { : "echo" }
        }
    };
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> Response {
    html! {
        form(method="POST") {
            p {
            : "Type something: ";
                input(type="text", name="echo");
                input(type="submit");
            }
        }
    }
}

fn post(req: Request) -> Response {
    let echoed = req.form("echo").unwrap_or("You didn't say anything!");
    html! {
        h1: echoed;
    }
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
