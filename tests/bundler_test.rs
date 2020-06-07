use vial::bundler::walk;

#[test]
fn test_walk() {
    assert_eq!(6, walk("tests/assets/").count());

    let expected = vec![
        "dinner.jpg",
        "letter.jpg",
        "puff.gif",
        "rfcs/rfc1288.txt",
        "rfcs/rfc1436.txt",
        "xiii.txt",
    ]
    .iter()
    .map(|s| format!("tests/assets/{}", s))
    .collect::<Vec<_>>();

    let mut actual = walk("tests/assets/")
        .map(|s| s.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    actual.sort();
    let mut actual = actual.iter();

    for file in expected {
        assert_eq!(file, *actual.next().unwrap());
    }

    assert_eq!(None, actual.next());
}
