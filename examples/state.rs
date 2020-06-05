use std::sync::atomic::{AtomicUsize, Ordering};
use vial::prelude::*;

routes! {
    GET "/" => hello;
    GET "/count" => count;
}

fn hello(req: Request) -> impl Responder {
    req.state::<HitCount>().0.fetch_add(1, Ordering::Relaxed);
    format!("Hits: {}", count(req))
}

fn count(req: Request) -> String {
    req.state::<HitCount>()
        .0
        .load(Ordering::Relaxed)
        .to_string()
}

#[derive(Default)]
struct HitCount(AtomicUsize);

fn main() {
    use_state!(HitCount::default());
    run!().unwrap();
}
