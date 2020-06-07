use vial::{Error, Responder, Response};

#[test]
fn from_response() {
    let res = Response::from_text("hi there").to_response();
    assert_eq!("hi there", res.body());
    assert_eq!(200, res.code());
    assert_eq!("text/plain; charset=utf8", res.content_type());
}

#[test]
fn from_string() {
    let res = "<h1>Welcome</h1>".to_string().to_response();
    assert_eq!("<h1>Welcome</h1>", res.body());
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
}

#[test]
fn from_str() {
    let res = "<h1>Welcome</h1>".to_response();
    assert_eq!("<h1>Welcome</h1>", res.body());
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
}

#[test]
fn from_usize() {
    let res = 422.to_response();
    assert_eq!("", res.body());
    assert_eq!(422, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
}

#[test]
fn from_result() {
    let ok: Result<&str, vial::Error> = Ok("<b>heya</b>");
    let res = ok.to_response();
    assert_eq!("<b>heya</b>", res.body());
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());

    let err: Result<&str, vial::Error> = Err(Error::ParseError);
    let res = err.to_response();
    assert_eq!("<h1>500 Internal Error</h1><pre>ParseError", res.body());
    assert_eq!(500, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
}

#[test]
fn from_option() {
    let some = Some("<h1>Welcome</h1>");
    let res = some.to_response();
    assert_eq!("<h1>Welcome</h1>", res.body());
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());

    let none: Option<&str> = None;
    let res = none.to_response();
    assert_eq!("404 Not Found", res.body());
    assert_eq!(404, res.code());
}
