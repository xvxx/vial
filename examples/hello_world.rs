use vial::prelude::*;

routes! {
    GET "/hi/world" => |_| "Hello, world!";
    GET "/hey/:place" => |req|
        format!("Heyo, {}!", req.arg("place").unwrap_or("?"));

    GET "/sleep" => |_| {
        std::thread::sleep(std::time::Duration::from_secs(5));
        "Zzzzz..."
    };

    GET "/" => welcome;
}

fn welcome(_req: Request) -> impl Responder {
    Response::from_file("examples/welcome.html")
}

fn main() {
    vial::run!().unwrap();
}
