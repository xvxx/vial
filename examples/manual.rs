use vial::prelude::*;

routes! {
    GET "/" => hello_world;
    POST "/" => redirect_to_greeting;
    GET "/:name" => hello_name;
}

fn hello_world(_req: Request) -> impl Responder {
    "<h1>Hello, world!</h1>
    <p><strong>What's your name?</strong></p>
    <form method='POST' action='/'>
        <p><input name='name' type='text'/></p>
        <p><input type='submit'/></p>
    </form>"
}

fn redirect_to_greeting(req: Request) -> Option<impl Responder> {
    let name = req.form("name")?;
    Some(Response::redirect_to(format!("/{}", name)))
}

fn hello_name(req: Request) -> impl Responder {
    format!("<h1>Why hello there, {}!</h1>", req.arg("name").unwrap())
}

fn main() {
    run!().unwrap();
}
