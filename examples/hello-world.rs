use vial::{vial, Request, Response};

vial! {
    GET "/hi/world" => |_| "Hello, world!".into();

    GET "/info" => |req| {
        format!("<h1>Request Information:</h1><pre>{:?}</p>", req).into()
    };

    GET "/" => welcome;
}

fn welcome(_req: Request) -> Response {
    Response::from(200).with_body("Welcome to your first drop of vial.")
}

fn main() {
    if let Err(e) = vial::run!("0.0.0.0:7667") {
        eprintln!("error: {}", e);
    }
}
