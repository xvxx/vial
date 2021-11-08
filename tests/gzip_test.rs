use std::fs;
use vial::{http_parser::parse, Request};

////
// helpers

fn fixture(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    match parse(fixture(name).as_bytes().to_vec()) {
        Ok(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}

#[test]
#[cfg(feature = "compression")]
fn gzip_header_test() {
    let request = Request::from_reader(fs::File::open("tests/http/gzip_GET.txt").unwrap()).unwrap();
    assert_eq!(request.compression(), Some(Compression::Gzip));
}
