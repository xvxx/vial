use vial::{self, asset};

#[test]
fn asset_exists() {
    vial::asset_dir!("./tests/assets/");

    assert!(asset::exists("dinner.jpg"));
    assert!(asset::exists("letter.jpg"));
    assert!(asset::exists("puff.gif"));
    assert!(!asset::exists("pooooof.gif"));
}

#[test]
fn etag_test() {
    vial::asset_dir!("./tests/assets/");
    assert_ne!(asset::etag("dinner.jpg"), asset::etag("letter.jpg"));
    assert_ne!(asset::etag("dinner.jpg"), asset::etag("made-up.gif"));
}

#[test]
fn normalize_path_test() {
    vial::asset_dir!("./tests/assets/");

    assert_eq!(
        Some("./tests/assets/dinner.jpg".to_string()),
        asset::normalize_path("./dinner.jpg")
    );
    assert_eq!(
        Some("./tests/assets/dinner.jpg".to_string()),
        asset::normalize_path("../dinner.jpg")
    );
    assert_eq!(
        Some("./tests/assets/./dinner.jpg".to_string()),
        asset::normalize_path("../../dinner.jpg")
    );
    assert_eq!(
        Some("./tests/assets/rfcs/rfc1436.txt".to_string()),
        asset::normalize_path("rfcs/rfc1436.txt")
    );
    assert_eq!(
        Some("./tests/assets/rfcs/rfc1436.txt".to_string()),
        asset::normalize_path("../rfcs/rfc1436.txt")
    );
    assert_eq!(
        Some("./tests/assets/./rfcs/rfc1436.txt".to_string()),
        asset::normalize_path("./../rfcs/rfc1436.txt")
    );
    assert_eq!(
        Some("./tests/assets/rfcs/rfc1288.txt".to_string()),
        asset::normalize_path("./rfcs/rfc1288.txt")
    );
}

#[test]
fn to_string_test() {
    vial::asset_dir!("./tests/assets/");

    assert_eq!(
        include_str!("assets/xiii.txt"),
        asset::to_string("xiii.txt").unwrap()
    );
}
