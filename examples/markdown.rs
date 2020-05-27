use vial::prelude::*;

routes! {
    GET "/" => |_| Response::from_file("README.md").as_markdown();
}

fn main() {
    vial::run!();
}
