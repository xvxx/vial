use vial::prelude::*;

routes! {
    GET "/" => get_ip;
}

fn get_ip(req: Request) -> impl Responder {
    req.remote_addr().to_string()
}

fn main() {
    run!().unwrap();
}
