use vial::prelude::*;

routes! {
    GET "/" => index;
}

fn index(req: Request) -> impl Responder {
    let count = req.cookie("count").unwrap_or_else(|| "0".into());
    let count = count.parse::<usize>().unwrap() + 1;
    let count = count.to_string();
    Response::from_cookie("count", &count).with_body(format!("Count: {}", count))
}

fn main() {
    run!().unwrap();
}
