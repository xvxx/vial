use vial::{vial, Request, Response};

vial! {
    GET "/hi/world" => |_| "Hello, world!".into();
    GET "/hey/world" => |_| "Heyo, world-o!".into();

    GET "/" => welcome;
}

fn welcome(_req: Request) -> Response {
    Response::from(200).from_file("examples/welcome.html")
}

fn main() {
    if let Err(e) = vial::run!("0.0.0.0:7667") {
        eprintln!("error: {}", e);
    }
}
