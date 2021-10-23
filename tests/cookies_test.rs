#![allow(non_snake_case)]

use std::{fs, fs::File};
use vial::{
    http_parser::{parse, Status},
    Error, Request,
};

////
// helpers

fn fixture(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    let x = parse(fixture(name).as_bytes().to_vec());
    println!("{:?}", x);

    match x.unwrap() {
        Status::Complete(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}

////
// tests

#[test]
#[cfg(feature = "cookies")]
fn assign_only_once() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_assign_only_once.txt").unwrap())
            .unwrap();
    assert_eq!("%1", request.cookie("foo").unwrap());
    assert_eq!("bar", request.cookie("bar").unwrap());
}

#[test]
#[cfg(feature = "cookies")]
fn basic() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_basic.txt").unwrap()).unwrap();
    assert_eq!("bar", request.cookie("foo").unwrap());
}
#[test]
#[cfg(feature = "cookies")]
fn escaping() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_escaping.txt").unwrap()).unwrap();
    assert_eq!(
        "bar=123456789&name=Magic+Mouse",
        request.cookie("foo").unwrap()
    );
}
#[test]
#[cfg(feature = "cookies")]
fn ignore_escaping_error_and_return_orig_value() {
    let request = Request::from_reader(
        File::open("tests/http/cookies_ignore_escaping_error_and_return_orig_value.txt").unwrap(),
    )
    .unwrap();
    assert_eq!("%1", request.cookie("foo").unwrap());
    assert_eq!("bar", request.cookie("bar").unwrap());
}
#[test]
#[cfg(feature = "cookies")]
fn ignore_non_values() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_ignore_non_values.txt").unwrap())
            .unwrap();
    assert_eq!("%1", request.cookie("foo").unwrap());
    assert_eq!("bar", request.cookie("bar").unwrap());
}

#[test]
#[cfg(feature = "cookies")]
fn unencoded() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_unencoded.txt").unwrap()).unwrap();
    assert_eq!(
        "bar=123456789&name=Magic+Mouse",
        request.cookie("foo").unwrap()
    );
}
#[test]
#[cfg(feature = "cookies")]
fn unencoded_2() {
    let request =
        Request::from_reader(File::open("tests/http/cookies_unencoded_2.txt").unwrap()).unwrap();
    assert_eq!("%20%22%2c%3b%2f", request.cookie("email").unwrap());
}

// These tests seem to be broken completely; todo: check with `basic-cookies` naked
// and see if it's an issue here or upstream. It claims to be RFC 6265-"compatible".
// #[test]
// #[cfg(feature = "cookies")]
// fn missing_value(){
//     let request = Request::from_reader(File::open("tests/http/cookies_missing_value.txt").unwrap()).unwrap();
//     println!("{:?}", request.cookies);
//     assert_eq!("1", request.cookie("bar").unwrap());
//     assert!(request.cookie("fizz").is_none());
//     assert_eq!("2", request.cookie("buzz").unwrap());
// }
// #[test]
// #[cfg(feature = "cookies")]
// fn ignore_spaces(){
//     let request = Request::from_reader(File::open("tests/http/cookies_ignore_spaces.txt").unwrap()).unwrap();
//     println!("{:?}", request.cookies);
//     assert_eq!("%1", request.cookie("foo").unwrap());
//     assert_eq!("raz", request.cookie("baz").unwrap());
// }
