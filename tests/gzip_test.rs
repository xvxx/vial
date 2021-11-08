use std::{fs, fs::File};
use vial::{http_parser::parse, Error, Request};

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
    let request = Request::from_reader(File::open("tests/http/gzip_GET.txt").unwrap()).unwrap();
    assert!(request.gzip());
}
