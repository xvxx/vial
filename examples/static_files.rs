vial::routes! {
    GET "/" => |_| "<img src='examples/doctor.png'/>";
}

fn main() {
    vial::asset_dir!(".");
    vial::run!().unwrap();
}
