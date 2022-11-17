#[cfg(any(feature = "json_serde", feature = "json_nano"))]
use {std::fs::File, vial::Request, vial::Response};

#[test]
#[cfg(feature = "json_serde")]
fn with_json() {
    let res = Response::from(200).with_json(serde_json::json!({"hello": "world"}));
    assert_eq!("application/json", res.content_type());
    assert_eq!("{\"hello\":\"world\"}", res.body());
}

#[test]
#[cfg(feature = "json_nano")]
fn with_json() {
    use nanoserde::SerJson;

    #[derive(SerJson)]
    struct Sample {
        hello: String,
    }
    let res = Response::from(200).with_json(Sample {
        hello: "world".to_owned(),
    });
    assert_eq!("application/json", res.content_type());
    assert_eq!("{\"hello\":\"world\"}", res.body());
}

#[test]
#[cfg(feature = "json_serde")]
fn json() {
    let req = Request::from_reader(File::open("tests/http/json_POST.txt").unwrap()).unwrap();
    assert_eq!(
        serde_json::json!({"hello": "world"}),
        req.json::<serde_json::Value>().unwrap()
    );
    let req = Request::from_reader(File::open("tests/http/bad_json_POST.txt").unwrap()).unwrap();
    assert!(req.json::<serde_json::Value>().is_err());
}

#[test]
#[cfg(feature = "json_nano")]
fn json() {
    use nanoserde::DeJson;

    let req = Request::from_reader(File::open("tests/http/json_POST.txt").unwrap()).unwrap();
    #[derive(DeJson)]
    struct Sample {
        hello: String,
    }

    assert_eq!("world".to_string(), req.json::<Sample>().unwrap().hello);
    let req = Request::from_reader(File::open("tests/http/bad_json_POST.txt").unwrap()).unwrap();
    assert!(req.json::<Sample>().is_err());
}
