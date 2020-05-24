use std::io;
use vial::asset;
use vial::prelude::*;

vial! {
    GET "/" => boom;
}

fn boom(_: Request) -> impl Responder {
    yay_or_nay()
}

fn yay_or_nay() -> Result<String, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Never."))
}

fn main() {
    vial::run!().unwrap();
}
