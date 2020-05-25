use vial::prelude::*;

vial! {
    GET "/" => |_| "<img src='examples/doctor.png'/>";
}

fn main() {
    vial::static_dir!(".");
    vial::run!().unwrap();
}
