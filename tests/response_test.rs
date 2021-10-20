use vial::Response;

#[test]
fn set_header() {
    let mut res = Response::new();
    res.set_header("Content-Type", "application/json");
    assert_eq!("application/json", res.header("Content-Type").unwrap());
    assert_eq!(None, res.header("Content-No-Type"));
    res.set_header("Content-Type", "text/plain; charset=utf8");
    assert_eq!(
        "text/plain; charset=utf8",
        res.header("Content-Type").unwrap()
    );
}

#[test]
fn from_header() {
    let v = "12345";
    let res1 = Response::from_header("X-Time", v);
    let res2 = Response::new().with_header("X-Time", v);

    assert_eq!(Some(v), res1.header("X-Time"));
    assert_eq!(res1.header("X-Time"), res2.header("X-Time"));
}

#[test]
fn from_asset() {
    vial::asset_dir!("tests/assets/");
    let res1 = Response::from_asset("puff.gif");
    assert_eq!(200, res1.code());
    assert_eq!("image/gif", res1.content_type());

    let res = Response::from_asset("something-fake.gif");
    assert_eq!(404, res.code());

    let res2 = Response::new().with_asset("puff.gif");
    assert_eq!(res1, res2);
}

#[test]
fn from_file() {
    let res1 = Response::from_file("README.md");
    assert_eq!(200, res1.code());
    assert_eq!("text/plain; charset=utf8", res1.content_type());

    let res = Response::from_file("something-fake.gif");
    assert_eq!(404, res.code());

    let res2 = Response::new().with_file("README.md");
    assert_eq!(res1, res2);
}

#[test]
fn from_reader() {
    use std::fs::File;

    let res1 = Response::from_reader(Box::new(File::open("README.md").unwrap()));
    assert_eq!(200, res1.code());
    assert_eq!("text/html; charset=utf8", res1.content_type());

    let res2 = Response::new().with_reader(Box::new(File::open("README.md").unwrap()));
    assert_eq!(res1, res2);

    let mut out: Vec<u8> = vec![];
    let date = format!("Date: {}", vial::util::http_current_date());
    let version = format!("Server: ~ vial {} ~", vial::VERSION);
    let expected = vec![
        "HTTP/1.1 200 OK",
        &version,
        &date,
        "Connection: close",
        "content-length: 0",
        "content-type: text/html; charset=utf8",
    ];

    res1.write(&mut out).unwrap();
    let out = String::from_utf8_lossy(&out);
    let lines = out.split("\r\n");
    let mut line_count = 0;
    for (i, line) in lines.enumerate() {
        // only care about headers
        if line.is_empty() {
            break;
        }
        line_count = i + 1;
        if !expected.is_empty() {
            assert!(
                expected.contains(&line),
                "expected {:?} in {:?}",
                line,
                expected
            );
        }
    }
    assert_eq!(expected.len(), line_count);
}

#[test]
fn from_body() {
    let res1 = Response::from_body("<h1>VialWeb</h1>");
    assert_eq!(200, res1.code());
    assert_eq!("text/html; charset=utf8", res1.content_type());

    let res2 = Response::new().with_body("<h1>VialWeb</h1>");
    assert_eq!(200, res2.code());
    assert_eq!(res1, res2);
}

#[test]
fn from_error() {
    use std::fs::File;

    let res1 = Response::from_error(File::open("doesnt-exist.txt").unwrap_err());
    assert_eq!(500, res1.code());
    assert_eq!("text/html; charset=utf8", res1.content_type());

    let res2 = Response::new().with_error(File::open("doesnt-exist.txt").unwrap_err());
    assert_eq!(500, res2.code());
    assert_eq!(res1, res2);
}

#[test]
fn from_text() {
    let res1 = Response::from_text("VialWeb");
    assert_eq!(200, res1.code());
    assert_eq!("text/plain; charset=utf8", res1.content_type());

    let res2 = Response::new().with_text("VialWeb");
    assert_eq!(200, res2.code());
    assert_eq!("text/plain; charset=utf8", res2.content_type());
    assert_eq!(res1, res2);
}

#[test]
fn from_code() {
    let res = Response::from_code(404);
    assert_eq!(404, res.code());

    let res = Response::from_code(402);
    assert_eq!(402, res.code());

    let res = Response::from_code(200);
    assert_eq!(200, res.code());
    assert_eq!("0", res.header("Content-Length").unwrap());

    let res1 = Response::from_code(501);
    let res2 = Response::new().with_code(501);
    assert_eq!(res1, res2);
}

#[test]
fn body_len() {
    let res = Response::from(200);
    assert_eq!(200, res.code());
    assert!(res.is_empty());

    let res = Response::from(404);
    assert_eq!(404, res.code());
    assert!(!res.is_empty());
    assert_eq!(13, res.len());

    let res = Response::from_file("tests/assets/dinner.jpg");
    assert_eq!(200, res.code());
    assert_eq!(1062724, res.len());

    vial::asset_dir!("tests/assets/");
    let res = Response::from_asset("puff.gif");
    assert_eq!(200, res.code());
    assert_eq!(771638, res.len());
}

#[test]
fn redirect_to() {
    let res = Response::redirect_to("/login");
    assert_eq!(302, res.code());
    assert_eq!("/login", res.header("Location").unwrap());
    assert_eq!("", res.body());

    let res = Response::redirect_to("https://google.com/");
    assert_eq!("https://google.com/", res.header("Location").unwrap());
    assert_eq!(302, res.code());
}

#[test]
fn write_response() {
     // TODO
}

#[test]
fn from() {
    let res = Response::from("<h1>VialWeb</h1>".to_string());
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
    assert_eq!("<h1>VialWeb</h1>", res.body());

    let res = Response::from("<h1>VialWeb</h1>");
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
    assert_eq!("<h1>VialWeb</h1>", res.body());

    let res = Response::from(404);
    assert_eq!(404, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
    assert_eq!("404 Not Found", res.body());

    let res = Response::from(500);
    assert_eq!(500, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
    assert_eq!("500 Internal Server Error", res.body());

    let res = Response::from(200);
    assert_eq!(200, res.code());
    assert_eq!("text/html; charset=utf8", res.content_type());
    assert_eq!("", res.body());
}

#[test]
#[cfg(feature = "json_serde")]
fn with_json() {
    let res = Response::from(200).with_json(serde_json::json!({"hello": "world"}));
    assert_eq!("application/json", res.content_type());
    assert_eq!("{\"hello\":\"world\"}", res.body());
}

#[test]
#[cfg(feature = "cookies")]
fn test_cookies() {
    let mut res = Response::new();
    res.set_cookie("Count", "2");
    assert_eq!("2", res.cookie("Count").unwrap());
    let mut out = vec![];
    res.write(&mut out).unwrap();
    let out = String::from_utf8_lossy(&out);
    assert!(out.contains("\r\nSet-Cookie: count=2\r\n"));

    let mut res = Response::new();
    res.remove_cookie("Count");
    let mut out = vec![];
    res.write(&mut out).unwrap();
    let out = String::from_utf8_lossy(&out);
    assert!(out.contains("\r\nSet-Cookie: count=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n"));
}
