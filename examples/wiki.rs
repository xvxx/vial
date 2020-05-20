use vial;

mod wiki {
    use vial::{vial, Request, Response};

    vial! {
        GET "/" => index;
        GET "/about" => about;

        GET "/new" => new;
        POST "/new" => create;

        GET "/:page/edit" => edit;
        POST "/:page" => update;
        GET "/:page" => show;
        GET "/:page.md" => show_raw;
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
}

pub use wiki::vial_recognize;

fn main() {
    vial::run!("0.0.0.0:4567", wiki).unwrap();
}
