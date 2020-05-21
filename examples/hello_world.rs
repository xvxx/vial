use vial::{vial, Request, Response};

vial! {
    GET "/hi/world" => |_| "Hello, world!".into();
    GET "/hey/world" => |_| "Heyo, world-o!".into();

    GET "/" => welcome;
}

fn welcome(_req: Request) -> Response {
    Response::from_file("examples/welcome.html")
}

fn main() {
    vial::run!().unwrap();
}
