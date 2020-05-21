use vial::vial;

vial! {
    GET "/hello/:name" => |req| {
        let name = req.arg("name").unwrap_or("");
        format!("Hello, {}", name).into()
    };
}

fn main() {
    vial::run!();
}
