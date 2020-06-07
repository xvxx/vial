use vial::prelude::*;

routes! {
    GET "/" => index;
    GET "/asset" => asset;
    GET "/body" => body;
    GET "/file" => file;
    GET "/text" => text;
    GET "/img/drink-me.jpeg" => drinkme;
    GET "/404" => not_found;
}

fn index(_req: Request) -> impl Responder {
    "<ul>
        <li><a href='/asset'>asset</a></li>
        <li><a href='/body'>body</a></li>
        <li><a href='/file'>file</a></li>
        <li><a href='/text'>text</a></li>
        <li><a href='/404'>404</a></li>
    </ul>"
}

fn asset(_req: Request) -> impl Responder {
    "<img src='doctor.png'/>"
}

fn drinkme(_req: Request) -> impl Responder {
    Response::from_file("docs/img/drink-me.jpeg")
}

fn body(_req: Request) -> impl Responder {
    Response::from_body(
        "<h3>Body</h3>
        <ul>
            <li>One</li>
            <li>Two</li>
            <li>Three</li>
        </ul>",
    )
}

fn file(_req: Request) -> impl Responder {
    Response::from_file("docs/index.html")
}

fn text(_req: Request) -> impl Responder {
    Response::from_text("## Plain Text\n\n- One\n- Two\n- Three")
}

fn not_found(_req: Request) -> impl Responder {
    Response::from(404).with_body("<h1>404 Not Found</h1>")
}

fn main() {
    asset_dir!("examples/assets");
    run!().unwrap();
}
