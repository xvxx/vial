use vial;

mod blog;
mod wiki;

mod index {
    use vial::vial;
    vial! {
        GET "/" => |_| "<h1>This is the index.</h1>";
    }
}

fn main() {
    vial::run!(index, wiki, blog).unwrap();
}
