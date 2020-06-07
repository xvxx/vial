use {
    std::{fs, path::Path},
    vial::prelude::*,
};

routes! {
    GET "/" => index;
    GET "/*file" => show;
}

fn index(_req: Request) -> impl Responder {
    "Files:
    <ul>
        <li><a href='/Cargo.toml'>Cargo.toml</a></li>
        <li><a href='/LICENSE-APACHE'>LICENSE-APACHE</a></li>
        <li><a href='/LICENSE-MIT'>LICENSE-MIT</a></li>
        <li><a href='/Makefile'>Makefile</a></li>
        <li><a href='/README.md'>README.md</a></li>
    </ul>"
}

fn show(req: Request) -> Option<Response> {
    let file = req.arg("file")?;
    println!("FILE: {}", file);
    if Path::new(file).exists() {
        Some(Response::from_text(fs::read_to_string(file).unwrap()))
    } else {
        None
    }
}

fn main() {
    run!().unwrap();
}
