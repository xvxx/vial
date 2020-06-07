#![allow(unused)]
use vial;

mod wiki {
    use vial::prelude::*;

    routes! {
        GET "/" => index;
        GET "/about" => about;

        GET "/new" => new;
        POST "/new" => create;

        GET "/:page/edit" => edit;
        POST "/:page" => update;
        GET "/:page" => show;
        GET "/:page.md" => show_raw;
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
}
