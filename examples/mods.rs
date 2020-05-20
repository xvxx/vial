use vial;

mod blog;
mod wiki;

mod index {
    use vial::{vial, Response};
    vial! {
        GET "/" => |_| Response::from("<h1>This is the index.</h1>");
    }
}

fn main() {
    vial::run!("0.0.0.0:7667", index, wiki, blog).unwrap();
}
