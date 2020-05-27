#![allow(unused)]
use vial::prelude::*;

routes! {
    GET "/blog" => index;
    GET "/blog/about" => about;

    GET "/blog/new" => new;
    POST "/blog/new" => create;

    GET "/blog/:page/edit" => edit;
    POST "/blog/:page" => update;
    GET "/blog/:page" => show;
    GET "/blog/:page.md" => show_raw;
}

fn index(req: Request) -> impl Responder {
    ""
}

fn about(req: Request) -> impl Responder {
    ""
}

fn new(req: Request) -> impl Responder {
    ""
}

fn create(req: Request) -> impl Responder {
    ""
}

fn edit(req: Request) -> impl Responder {
    ""
}

fn update(req: Request) -> impl Responder {
    ""
}

fn show(req: Request) -> impl Responder {
    ""
}

fn show_raw(req: Request) -> impl Responder {
    ""
}

fn main() {
    vial::run!().unwrap();
}
