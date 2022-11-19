#[test]
#[cfg(feature = "sessions")]
fn session_encode() {
    use vial::session::Session;
    let session = Session::new("secret-key");
    let encoded = session.encode("hello");
    assert_eq!(encoded, "mYcOuCZs");
}

#[test]
#[cfg(feature = "sessions")]
fn session_decode() {
    use vial::session::Session;
    let session = Session::new("secret-key");
    let decoded = session.decode("mYcOuCZs");
    assert_eq!(decoded, Ok("hello".to_string()));
}
