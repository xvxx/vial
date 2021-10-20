mod mods;
use mods::{blog, wiki};

mod index {
    vial::routes! {
        GET "/" => |_| "<h1>This is the index.</h1>";
    }
}

fn main() {
    vial::run!(index, wiki, blog).unwrap();
}
