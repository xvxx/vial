use std::sync::atomic::{AtomicUsize, Ordering};
use vial::prelude::*;

routes! {
    // `count` will run before all routes in this block
    #![filter(count)]

    GET "/" => |_| "Hey there!";
    GET "/hits" => hits;

    // `count` will run again when /double is visited
    #[filter(count)]
    GET "/double" => double;

    // `echo` will be called when /echo is visited
    #[filter(echo)]
    GET "/echo" => |_| "Is there an echo in here?";
}

fn hits(req: Request) -> impl Responder {
    format!("Hits: {}", req.counter().count())
}

fn double(req: Request) -> impl Responder {
    "Double trouble."
}

fn echo(req: &mut Request) -> Option<Response> {
    println!("{:#?}", req);
    None
}

fn count(req: &mut Request) -> Option<Response> {
    req.counter().incr();
    None
}

#[derive(Debug, Default)]
struct Counter(AtomicUsize);

impl Counter {
    fn count(&self) -> String {
        self.0.load(Ordering::Relaxed).to_string()
    }

    fn incr(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

trait WithCounter {
    fn counter(&self) -> &Counter;
}

impl WithCounter for Request {
    fn counter(&self) -> &Counter {
        self.state::<Counter>()
    }
}

fn main() {
    use_state!(Counter::default());
    run!().unwrap();
}
