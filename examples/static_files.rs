use vial::prelude::*;

vial! {
    GET "/" => |_| "<img src='examples/doctor.png'/>";
}

fn main() {
    vial::run!().unwrap();
}
