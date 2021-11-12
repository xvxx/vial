use {
    std::sync::atomic::{AtomicUsize, Ordering},
    vial::prelude::*,
};

routes! {
    // `count` will run before all routes in this block
    #![filter(count)]

    GET "/" => |_| "Hey there!";
    GET "/hits" => hits;

    // `count` will run again when /double is visited
    #[filter(count)]
    GET "/double" => double;

    // `echo` will be called when /echo is visited, as well as `count`
    // because it's a module-level filter
    #[filter(echo)]
    GET "/echo" => |_| "Is there an echo in here?";
}

fn hits(req: Request) -> impl Responder {
    format!("Hits: {}", req.counter().count())
}

fn double(_req: Request) -> impl Responder {
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

// We do this purely for the convenience of using `req.counter()`
// instead of `req.state::<Counter>()`.
// Nov 12 2021: not quite sure which clippy triggered this edit but now it's `vial::Request::state` not `self.state`
trait WithCounter {
    fn counter(&self) -> &Counter;
}

impl WithCounter for Request {
    fn counter(&self) -> &Counter {
        vial::Request::state::<Counter>()
    }
}

fn main() {
    use_state!(Counter::default());
    run!().unwrap();
}
