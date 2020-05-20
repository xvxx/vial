use vial;

mod my_routes;

fn main() {
    vial::run!("0.0.0.0:4567", my_routes).unwrap();
}
