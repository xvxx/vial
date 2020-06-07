use vial::Method;

#[test]
fn converts_from_str() {
    assert_eq!(Method::from("GET"), Method::GET);
    assert_eq!(Method::from("HEAD"), Method::HEAD);
    assert_eq!(Method::from("POST"), Method::POST);
    assert_eq!(Method::from("PUT"), Method::PUT);
    assert_eq!(Method::from("DELETE"), Method::DELETE);
    assert_eq!(Method::from("PATCH"), Method::PATCH);
    assert_eq!(Method::from("OPTIONS"), Method::OPTIONS);
    assert_eq!(Method::from("TRACE"), Method::TRACE);
    assert_eq!(Method::from("INVALID"), Method::GET);
}
