use vial::prelude::*;

vial! {
    GET "/hi/world" => |_| "Hello, world!";
    GET "/hey/world" => |_| "Heyo, world-o!";

    GET "/" => welcome;
}

fn welcome(_req: Request) -> impl Responder {
    Response::from_file("examples/welcome.html")
}

fn main() {
    vial::run!().unwrap();
}
