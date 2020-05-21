use vial::vial;

vial! {
    GET "/" => |_| "Hello, world!";
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
