#![allow(unused)]

vial::routes! {
    GET "/hello/:id/:name" => |req| {
        let name = req.arg("name").unwrap_or("");
        format!("Hello, {}. You're ID #{}", name, req.arg("id").unwrap_or("0"))
    };

    GET "/page/:name" => |req| {
        format!("Page: {}", req.arg("name").unwrap_or("?"))
    };

    GET "/page/:name/edit" => |req| {
        format!("Edit: {}", req.arg("name").unwrap_or("?"))
    };
}

fn main() {
    vial::run!();
}
