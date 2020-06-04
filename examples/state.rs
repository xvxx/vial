use std::sync::atomic::{AtomicUsize, Ordering};
use vial::prelude::*;

routes! {
    GET "/" => hello;
    GET "/count" => count;
}

fn hello(hit_count: State<HitCount>) -> impl Responder {
    hit_count.0.fetch_add(1, Ordering::Relaxed);
    format!("Hits: {}", count(hit_count))
}

fn count(hit_count: State<HitCount>) -> String {
    hit_count.0.load(Ordering::Relaxed).to_string()
}

struct HitCount(AtomicUsize);

fn main() {
    use_state!(HitCount(AtomicUsize::new(0)));
    run!().unwrap();
}
