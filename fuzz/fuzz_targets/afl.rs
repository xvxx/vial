use vial::Request;
extern crate afl;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        let x = vial::Request::from_reader(data);
    });
}
