use vial::{vial, Request, Response};

vial! {
    GET "/hi/world" => |_| "Hello, world!".into();
    GET "/" => echo;
}

fn echo(req: Request) -> Response {
    if let Some(msg) = req.param("echo") {
        Response::from(format!("You said: <b>{}</b>", msg))
    } else {
        Response::from(format!("<form action='' method='get'>Say something: <input type='text' name='echo'/> <input type='submit'/>"))
    }
}

fn main() {
    if let Err(e) = vial::run!("0.0.0.0:7667") {
        eprintln!("error: {}", e);
    }
}
