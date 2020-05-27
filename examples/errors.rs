use std::io;
use vial::prelude::*;

routes! {
    GET "/" => boom;
    GET "/err" => action_that_errs;
}

fn action_that_errs(_: Request) -> Result<impl Responder, io::Error> {
    let body = yay_or_nay()?;
    Ok(format!("act: {}", body))
}

fn boom(_: Request) -> impl Responder {
    yay_or_nay()
}

fn yay_or_nay() -> Result<String, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Never!"))
}

fn main() {
    vial::run!().unwrap();
}
