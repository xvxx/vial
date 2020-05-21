#![allow(unused)]
use vial::{vial, Request, Response};

vial! {
    GET "/blog" => index;
    GET "/blog/about" => about;

    GET "/blog/new" => new;
    POST "/blog/new" => create;

    GET "/blog/:page/edit" => edit;
    POST "/blog/:page" => update;
    GET "/blog/:page" => show;
    GET "/blog/:page.md" => show_raw;
}

fn index(req: Request) -> Response {
    "".into()
}

fn about(req: Request) -> Response {
    "".into()
}

fn new(req: Request) -> Response {
    "".into()
}

fn create(req: Request) -> Response {
    "".into()
}

fn edit(req: Request) -> Response {
    "".into()
}

fn update(req: Request) -> Response {
    "".into()
}

fn show(req: Request) -> Response {
    "".into()
}

fn show_raw(req: Request) -> Response {
    "".into()
}

fn main() {
    vial::run!().unwrap();
}
