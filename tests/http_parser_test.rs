#![allow(non_snake_case)]

use std::fs;
use vial::{
    http_parser::{parse, Status},
    Request,
};

////
// helpers

fn fixture(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    match parse(fixture(name).as_bytes().to_vec()).unwrap() {
        Status::Complete(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}

////
// tests

#[test]
fn parses_simple_GET() {
    let request = parse_fixture("tests/http/simple_GET.txt");
    assert_eq!("/hello.html", request.path());
    assert_eq!("GET", request.method());
    assert_eq!("GET", request.header("USER-AGENT").unwrap());
}

#[test]
fn parses_basic_GET() {
    let request = parse_fixture("tests/http/basic_GET.txt");
    assert_eq!("/", request.path());
    assert_eq!("GET", request.method());
    assert_eq!("www.codecademy.com", request.header("Host").unwrap());
}

#[test]
fn parses_big_GET() {
    let request = parse_fixture("tests/http/big_GET.txt");
    assert_eq!("/something", request.path());
    assert_eq!("GET", request.method());
}

#[test]
fn parses_simple_POST() {
    let request = parse_fixture("tests/http/simple_POST.txt");
    assert_eq!("/something", request.path());
    assert_eq!("POST", request.method());
    assert_eq!(Some("1234"), request.form("licenseID"));
    assert_eq!(Some("hi there"), request.form("content"));
    assert_eq!(Some("<abc></abc>"), request.form("paramsXML"));
    assert_eq!(None, request.form("something"));
}

#[test]
fn rejects_malformed_headers() {}

#[test]
fn rejects_expected_but_no_body() {}
