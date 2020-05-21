#[macro_use]
extern crate horrorshow;
use vial::{vial, Request, Response};

vial! {
    GET "/" => |_| {
        Response::from(
            html! {
                p {
                    : "You're looking for this: ";
                    a(href="/echo") { : "echo" }
                }
            }
        )
    };
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> Response {
    Response::from(html! {
        form(method="POST") {
            p {
            : "Type something: ";
                input(type="text", name="echo");
                input(type="submit");
            }
        }
    })
}

fn post(req: Request) -> Response {
    let echoed = req.form("echo").unwrap_or("You didn't say anything!");
    Response::from(html! {
        h1: echoed;
    })
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
