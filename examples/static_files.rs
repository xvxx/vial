use vial::prelude::*;

routes! {
    GET "/" => |_| "<img src='/doctor.png' />";
    GET "/welcome" => |_| Response::from_asset("welcome.html");
}

fn main() {
    asset_dir!("./examples/assets");
    run!().unwrap();
}
