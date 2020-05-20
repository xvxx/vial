use vial::vial;

vial! {
    GET "/vial" => |_| "What?".into();
}

mod v {
    use vial::vial;
    vial! {
        GET "/v" => |_| "v!".into();
    }
}

fn main() {
    vial::run!("0.0.0.0:7667", crate, v).unwrap();
}
