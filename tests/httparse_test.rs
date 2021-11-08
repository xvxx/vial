// httparse tests <3
// https://github.com/seanmonstar/httparse/blob/master/tests/uri.rs

use vial::{http_parser::parse, Error};

macro_rules! test {
    ($name:ident, $buf:expr, |$arg:ident| $body:expr) => {
        #[test]
        fn $name() {
            use vial::{http_parser::parse, Request};

            let req = match parse($buf.to_vec()) {
                Ok(request) => request,
                Err(e) => panic!("Expected Status::Complete, got Err - {})", e),
            };
            closure(req);

            fn closure($arg: Request) {
                $body
            }
        }
    };
    ($name:ident, $buf:expr, $err:expr) => {
        #[test]
        fn $name() {
            use vial::http_parser::parse;
            match parse($buf.to_vec()) {
                o @ Ok(..) => panic!("Expected http_parser::Error, got {:?}", o),
                Err(e) => assert_eq!(e, $err),
            }
        }
    };
}

test! {
    urltest_001,
    b"GET /bar;par?b HTTP/1.1\r\nHost: foo\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/bar;par?b");
        assert_eq!(req.header("Host").unwrap(), "foo");
    }
}

test! {
    urltest_002,
    b"GET /x HTTP/1.1\r\nHost: test\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/x");
        assert_eq!(req.header("Host").unwrap(), "test");
    }
}

test! {
    urltest_003,
    b"GET /x HTTP/1.1\r\nHost: test\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/x");
        assert_eq!(req.header("Host").unwrap(), "test");
    }
}

test! {
    urltest_004,
    b"GET /foo/foo.com HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/foo.com");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_005,
    b"GET /foo/:foo.com HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:foo.com");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_006,
    b"GET /foo/foo.com HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/foo.com");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_007,
    b"GET  foo.com HTTP/1.1\r\nHost: \r\n\r\n",
    Error::ParsePath
}

test! {
    urltest_008,
    b"GET /%20b%20?%20d%20 HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%20b%20?%20d%20");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_009,
    b"GET x x HTTP/1.1\r\nHost: \r\n\r\n",
    Error::ParseVersion
}

test! {
    urltest_010,
    b"GET /c HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_011,
    b"GET /c HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_012,
    b"GET /c HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_013,
    b"GET /c HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_014,
    b"GET /c HTTP/1.1\r\nHost: f\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c");
        assert_eq!(req.header("Host").unwrap(), "f");
    }
}

test! {
    urltest_015,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_016,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_017,
    b"GET /foo/:foo.com/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:foo.com/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_018,
    b"GET /foo/:foo.com/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:foo.com/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_019,
    b"GET /foo/: HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_020,
    b"GET /foo/:a HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:a");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_021,
    b"GET /foo/:/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_022,
    b"GET /foo/:/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_023,
    b"GET /foo/: HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_024,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_025,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_026,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_027,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_028,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_029,
    b"GET /foo/:23 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:23");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_030,
    b"GET /:23 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/:23");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_031,
    b"GET /foo/:: HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/::");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_032,
    b"GET /foo/::23 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/::23");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_033,
    b"GET /d HTTP/1.1\r\nHost: c\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/d");
        assert_eq!(req.header("Host").unwrap(), "c");
    }
}

test! {
    urltest_034,
    b"GET /foo/:@c:29 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/:@c:29");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_035,
    b"GET //@ HTTP/1.1\r\nHost: foo.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//@");
        assert_eq!(req.header("Host").unwrap(), "foo.com");
    }
}

test! {
    urltest_036,
    b"GET /b:c/d@foo.com/ HTTP/1.1\r\nHost: a\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/b:c/d@foo.com/");
        assert_eq!(req.header("Host").unwrap(), "a");
    }
}

test! {
    urltest_037,
    b"GET /bar.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/bar.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_038,
    b"GET /////// HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "///////");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_039,
    b"GET ///////bar.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "///////bar.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_040,
    b"GET //:///// HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//://///");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_041,
    b"GET /foo HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_042,
    b"GET /bar HTTP/1.1\r\nHost: foo\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/bar");
        assert_eq!(req.header("Host").unwrap(), "foo");
    }
}

test! {
    urltest_043,
    b"GET /path;a??e HTTP/1.1\r\nHost: foo\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/path;a??e");
        assert_eq!(req.header("Host").unwrap(), "foo");
    }
}

test! {
    urltest_044,
    b"GET /abcd?efgh?ijkl HTTP/1.1\r\nHost: foo\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/abcd?efgh?ijkl");
        assert_eq!(req.header("Host").unwrap(), "foo");
    }
}

test! {
    urltest_045,
    b"GET /abcd HTTP/1.1\r\nHost: foo\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/abcd");
        assert_eq!(req.header("Host").unwrap(), "foo");
    }
}

test! {
    urltest_046,
    b"GET /foo/[61:24:74]:98 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/[61:24:74]:98");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_047,
    b"GET /foo/[61:27]/:foo HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/[61:27]/:foo");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_048,
    b"GET /example.com/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_049,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_050,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_051,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_052,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_053,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_054,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_055,
    b"GET /foo/example.com/ HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/example.com/");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_056,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_057,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_058,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_059,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_060,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_061,
    b"GET /a/b/c HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a/b/c");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_062,
    b"GET /a/%20/c HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a/%20/c");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_063,
    b"GET /a%2fc HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a%2fc");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_064,
    b"GET /a/%2f/c HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a/%2f/c");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_065,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_066,
    b"GET text/html,test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "text/html,test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_067,
    b"GET 1234567890 HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "1234567890");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_068,
    b"GET /c:/foo/bar.html HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:/foo/bar.html");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_069,
    b"GET /c:////foo/bar.html HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:////foo/bar.html");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_070,
    b"GET /C:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_071,
    b"GET /C:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_072,
    b"GET /C:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_073,
    b"GET /file HTTP/1.1\r\nHost: server\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/file");
        assert_eq!(req.header("Host").unwrap(), "server");
    }
}

test! {
    urltest_074,
    b"GET /file HTTP/1.1\r\nHost: server\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/file");
        assert_eq!(req.header("Host").unwrap(), "server");
    }
}

test! {
    urltest_075,
    b"GET /file HTTP/1.1\r\nHost: server\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/file");
        assert_eq!(req.header("Host").unwrap(), "server");
    }
}

test! {
    urltest_076,
    b"GET /foo/bar.txt HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar.txt");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_077,
    b"GET /home/me HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/home/me");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_078,
    b"GET /test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_079,
    b"GET /test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_080,
    b"GET /tmp/mock/test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/tmp/mock/test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_081,
    b"GET /tmp/mock/test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/tmp/mock/test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_082,
    b"GET /foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_083,
    b"GET /.foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/.foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_084,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_085,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_086,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_087,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_088,
    b"GET /foo/..bar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/..bar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_089,
    b"GET /foo/ton HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/ton");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_090,
    b"GET /a HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_091,
    b"GET /ton HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/ton");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_092,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_093,
    b"GET /foo/%2e%2 HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/%2e%2");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_094,
    b"GET /%2e.bar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%2e.bar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_095,
    b"GET // HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_096,
    b"GET /foo/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_097,
    b"GET /foo/bar/ HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar/");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_098,
    b"GET /foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_099,
    b"GET /%20foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%20foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_100,
    b"GET /foo% HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_101,
    b"GET /foo%2 HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%2");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_102,
    b"GET /foo%2zbar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%2zbar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_103,
    b"GET /foo%2%C3%82%C2%A9zbar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%2%C3%82%C2%A9zbar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_104,
    b"GET /foo%41%7a HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%41%7a");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_105,
    b"GET /foo%C2%91%91 HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%C2%91%91");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_106,
    b"GET /foo%00%51 HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%00%51");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_107,
    b"GET /(%28:%3A%29) HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/(%28:%3A%29)");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_108,
    b"GET /%3A%3a%3C%3c HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%3A%3a%3C%3c");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_109,
    b"GET /foobar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foobar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_110,
    b"GET //foo//bar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//foo//bar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_111,
    b"GET /%7Ffp3%3Eju%3Dduvgw%3Dd HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%7Ffp3%3Eju%3Dduvgw%3Dd");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_112,
    b"GET /@asdf%40 HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/@asdf%40");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_113,
    b"GET /%E4%BD%A0%E5%A5%BD%E4%BD%A0%E5%A5%BD HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%E4%BD%A0%E5%A5%BD%E4%BD%A0%E5%A5%BD");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_114,
    b"GET /%E2%80%A5/foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%E2%80%A5/foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_115,
    b"GET /%EF%BB%BF/foo HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%EF%BB%BF/foo");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_116,
    b"GET /%E2%80%AE/foo/%E2%80%AD/bar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%E2%80%AE/foo/%E2%80%AD/bar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_117,
    b"GET /foo?bar=baz HTTP/1.1\r\nHost: www.google.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo?bar=baz");
        assert_eq!(req.header("Host").unwrap(), "www.google.com");
    }
}

test! {
    urltest_118,
    b"GET /foo?bar=baz HTTP/1.1\r\nHost: www.google.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo?bar=baz");
        assert_eq!(req.header("Host").unwrap(), "www.google.com");
    }
}

test! {
    urltest_119,
    b"GET test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_120,
    b"GET /foo%2Ehtml HTTP/1.1\r\nHost: www\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo%2Ehtml");
        assert_eq!(req.header("Host").unwrap(), "www");
    }
}

test! {
    urltest_121,
    b"GET /foo/html HTTP/1.1\r\nHost: www\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/html");
        assert_eq!(req.header("Host").unwrap(), "www");
    }
}

test! {
    urltest_122,
    b"GET /foo HTTP/1.1\r\nHost: www.google.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo");
        assert_eq!(req.header("Host").unwrap(), "www.google.com");
    }
}

test! {
    urltest_123,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_124,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_125,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_126,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_127,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_128,
    b"GET /example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_129,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_130,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_131,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_132,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_133,
    b"GET example.com/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "example.com/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_134,
    b"GET /test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_135,
    b"GET /test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_136,
    b"GET /test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_137,
    b"GET /test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_138,
    b"GET /aaa/test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/aaa/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_139,
    b"GET /test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_140,
    b"GET /%E4%B8%AD/test.txt HTTP/1.1\r\nHost: www.example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%E4%B8%AD/test.txt");
        assert_eq!(req.header("Host").unwrap(), "www.example.com");
    }
}

test! {
    urltest_141,
    b"GET /... HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/...");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_142,
    b"GET /a HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/a");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_143,
    b"GET /%EF%BF%BD?%EF%BF%BD HTTP/1.1\r\nHost: x\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%EF%BF%BD?%EF%BF%BD");
        assert_eq!(req.header("Host").unwrap(), "x");
    }
}

test! {
    urltest_144,
    b"GET /bar HTTP/1.1\r\nHost: example.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/bar");
        assert_eq!(req.header("Host").unwrap(), "example.com");
    }
}

test! {
    urltest_145,
    b"GET test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_146,
    b"GET x@x.com HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "x@x.com");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_147,
    b"GET , HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), ",");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_148,
    b"GET blank HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "blank");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_149,
    b"GET test?test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "test?test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_150,
    b"GET /%60%7B%7D?`{} HTTP/1.1\r\nHost: h\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/%60%7B%7D?`{}");
        assert_eq!(req.header("Host").unwrap(), "h");
    }

}

test! {
    urltest_151,
    b"GET /?%27 HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?%27");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_152,
    b"GET /?' HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?'");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_153,
    b"GET /some/path HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/some/path");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_154,
    b"GET /smth HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/smth");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_155,
    b"GET /some/path HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/some/path");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_156,
    b"GET /pa/i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_157,
    b"GET /i HTTP/1.1\r\nHost: ho\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "ho");
    }
}

test! {
    urltest_158,
    b"GET /pa/i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_159,
    b"GET /i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_160,
    b"GET /i HTTP/1.1\r\nHost: ho\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "ho");
    }
}

test! {
    urltest_161,
    b"GET /i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_162,
    b"GET /i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_163,
    b"GET /i HTTP/1.1\r\nHost: ho\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "ho");
    }
}

test! {
    urltest_164,
    b"GET /i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_165,
    b"GET /pa/pa?i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/pa?i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_166,
    b"GET /pa?i HTTP/1.1\r\nHost: ho\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa?i");
        assert_eq!(req.header("Host").unwrap(), "ho");
    }
}

test! {
    urltest_167,
    b"GET /pa/pa?i HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/pa?i");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_168,
    b"GET sd HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "sd");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_169,
    b"GET sd/sd HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "sd/sd");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_170,
    b"GET /pa/pa HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/pa");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_171,
    b"GET /pa HTTP/1.1\r\nHost: ho\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa");
        assert_eq!(req.header("Host").unwrap(), "ho");
    }
}

test! {
    urltest_172,
    b"GET /pa/pa HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pa/pa");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_173,
    b"GET /x HTTP/1.1\r\nHost: %C3%B1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/x");
        assert_eq!(req.header("Host").unwrap(), "%C3%B1");
    }
}

test! {
    urltest_174,
    b"GET \\.\\./ HTTP/1.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "\\.\\./");
    }
}

test! {
    urltest_175,
    b"GET :a@example.net HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), ":a@example.net");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_176,
    b"GET %NBD HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "%NBD");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_177,
    b"GET %1G HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "%1G");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_178,
    b"GET /relative_import.html HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/relative_import.html");
        assert_eq!(req.header("Host").unwrap(), "127.0.0.1");
    }
}

test! {
    urltest_179,
    b"GET /?foo=%7B%22abc%22 HTTP/1.1\r\nHost: facebook.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?foo=%7B%22abc%22");
        assert_eq!(req.header("Host").unwrap(), "facebook.com");
    }
}

test! {
    urltest_180,
    b"GET /jqueryui@1.2.3 HTTP/1.1\r\nHost: localhost\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/jqueryui@1.2.3");
        assert_eq!(req.header("Host").unwrap(), "localhost");
    }
}

test! {
    urltest_181,
    b"GET /path?query HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/path?query");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_182,
    b"GET /foo/bar?a=b&c=d HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar?a=b&c=d");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_183,
    b"GET /foo/bar??a=b&c=d HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar??a=b&c=d");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_184,
    b"GET /foo/bar HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_185,
    b"GET /baz?qux HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/baz?qux");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    urltest_186,
    b"GET /baz?qux HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/baz?qux");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    urltest_187,
    b"GET /baz?qux HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/baz?qux");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    urltest_188,
    b"GET /baz?qux HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/baz?qux");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    urltest_189,
    b"GET /baz?qux HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/baz?qux");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    urltest_190,
    b"GET /C%3A/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C%3A/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_191,
    b"GET /C%7C/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C%7C/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_192,
    b"GET /C:/Users/Domenic/Dropbox/GitHub/tmpvar/jsdom/test/level2/html/files/pix/submit.gif HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/Users/Domenic/Dropbox/GitHub/tmpvar/jsdom/test/level2/html/files/pix/submit.gif");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_193,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_194,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_195,
    b"GET /d: HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/d:");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_196,
    b"GET /d:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/d:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_197,
    b"GET /test?test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_198,
    b"GET /test?test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_199,
    b"GET /test?x HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?x");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_200,
    b"GET /test?x HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?x");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_201,
    b"GET /test?test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_202,
    b"GET /test?test HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?test");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_203,
    b"GET /?fox HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?fox");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_204,
    b"GET /localhost//cat HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/localhost//cat");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_205,
    b"GET /localhost//cat HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/localhost//cat");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_206,
    b"GET /mouse HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/mouse");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_207,
    b"GET /pig HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pig");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_208,
    b"GET /pig HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pig");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_209,
    b"GET /pig HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/pig");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_210,
    b"GET /localhost//pig HTTP/1.1\r\nHost: lion\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/localhost//pig");
        assert_eq!(req.header("Host").unwrap(), "lion");
    }
}

test! {
    urltest_211,
    b"GET /rooibos HTTP/1.1\r\nHost: tea\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/rooibos");
        assert_eq!(req.header("Host").unwrap(), "tea");
    }
}

test! {
    urltest_212,
    b"GET /?chai HTTP/1.1\r\nHost: tea\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?chai");
        assert_eq!(req.header("Host").unwrap(), "tea");
    }
}

test! {
    urltest_213,
    b"GET /C: HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_214,
    b"GET /C: HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_215,
    b"GET /C: HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_216,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_217,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_218,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_219,
    b"GET /dir/C HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/dir/C");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_220,
    b"GET /dir/C|a HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/dir/C|a");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_221,
    b"GET /c:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_222,
    b"GET /c:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_223,
    b"GET /c:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_224,
    b"GET /c:/foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/c:/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_225,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_226,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_227,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_228,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_229,
    b"GET /C:/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/C:/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_230,
    b"GET /?q=v HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/?q=v");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_231,
    b"GET ?x HTTP/1.1\r\nHost: %C3%B1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "?x");
        assert_eq!(req.header("Host").unwrap(), "%C3%B1");
    }
}

test! {
    urltest_232,
    b"GET ?x HTTP/1.1\r\nHost: %C3%B1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "?x");
        assert_eq!(req.header("Host").unwrap(), "%C3%B1");
    }
}

test! {
    urltest_233,
    b"GET // HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_234,
    b"GET //x/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "//x/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_235,
    b"GET /someconfig;mode=netascii HTTP/1.1\r\nHost: foobar.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/someconfig;mode=netascii");
        assert_eq!(req.header("Host").unwrap(), "foobar.com");
    }
}

test! {
    urltest_236,
    b"GET /Index.ut2 HTTP/1.1\r\nHost: 10.10.10.10\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/Index.ut2");
        assert_eq!(req.header("Host").unwrap(), "10.10.10.10");
    }
}

test! {
    urltest_237,
    b"GET /0?baz=bam&qux=baz HTTP/1.1\r\nHost: somehost\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/0?baz=bam&qux=baz");
        assert_eq!(req.header("Host").unwrap(), "somehost");
    }
}

test! {
    urltest_238,
    b"GET /sup HTTP/1.1\r\nHost: host\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/sup");
        assert_eq!(req.header("Host").unwrap(), "host");
    }
}

test! {
    urltest_239,
    b"GET /foo/bar.git HTTP/1.1\r\nHost: github.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar.git");
        assert_eq!(req.header("Host").unwrap(), "github.com");
    }
}

test! {
    urltest_240,
    b"GET /channel?passwd HTTP/1.1\r\nHost: myserver.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/channel?passwd");
        assert_eq!(req.header("Host").unwrap(), "myserver.com");
    }
}

test! {
    urltest_241,
    b"GET /foo.bar.org?type=TXT HTTP/1.1\r\nHost: fw.example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo.bar.org?type=TXT");
        assert_eq!(req.header("Host").unwrap(), "fw.example.org");
    }
}

test! {
    urltest_242,
    b"GET /ou=People,o=JNDITutorial HTTP/1.1\r\nHost: localhost\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/ou=People,o=JNDITutorial");
        assert_eq!(req.header("Host").unwrap(), "localhost");
    }
}

test! {
    urltest_243,
    b"GET /foo/bar HTTP/1.1\r\nHost: github.com\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/foo/bar");
        assert_eq!(req.header("Host").unwrap(), "github.com");
    }
}

test! {
    urltest_244,
    b"GET ietf:rfc:2648 HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "ietf:rfc:2648");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_245,
    b"GET joe@example.org,2001:foo/bar HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "joe@example.org,2001:foo/bar");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_246,
    b"GET /path HTTP/1.1\r\nHost: H%4fSt\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/path");
        assert_eq!(req.header("Host").unwrap(), "H%4fSt");
    }
}

test! {
    urltest_247,
    b"GET https://example.com:443/ HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "https://example.com:443/");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_248,
    b"GET d3958f5c-0777-0845-9dcf-2cb28783acaf HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "d3958f5c-0777-0845-9dcf-2cb28783acaf");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_249,
    b"GET /test?%22 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%22");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_250,
    b"GET /test HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_251,
    b"GET /test?%3C HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%3C");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_252,
    b"GET /test?%3E HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%3E");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_253,
    b"GET /test?%E2%8C%A3 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%E2%8C%A3");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_254,
    b"GET /test?%23%23 HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%23%23");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_255,
    b"GET /test?%GH HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?%GH");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_256,
    b"GET /test?a HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?a");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_257,
    b"GET /test?a HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?a");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    urltest_258,
    b"GET /test-a-colon-slash.html HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test-a-colon-slash.html");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_259,
    b"GET /test-a-colon-slash-slash.html HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test-a-colon-slash-slash.html");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_260,
    b"GET /test-a-colon-slash-b.html HTTP/1.1\r\nHost: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test-a-colon-slash-b.html");
        assert_eq!(req.header("Host").unwrap(), "");
    }
}

test! {
    urltest_261,
    b"GET /test-a-colon-slash-slash-b.html HTTP/1.1\r\nHost: b\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test-a-colon-slash-slash-b.html");
        assert_eq!(req.header("Host").unwrap(), "b");
    }
}

test! {
    urltest_262,
    b"GET /test?a HTTP/1.1\r\nHost: example.org\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/test?a");
        assert_eq!(req.header("Host").unwrap(), "example.org");
    }
}

test! {
    test_request_simple,
    b"GET / HTTP/1.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_simple_with_query_params,
    b"GET /thing?data=a HTTP/1.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/thing?data=a");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_simple_with_whatwg_query_params,
    b"GET /thing?data=a^ HTTP/1.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/thing?data=a^");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_headers,
    b"GET / HTTP/1.1\r\nHost: foo.com\r\nCookie: \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.header("Host").unwrap(), "foo.com");
        assert_eq!(req.header("Cookie").unwrap(), "");
    }
}

test! {
    test_request_headers_optional_whitespace,
    b"GET / HTTP/1.1\r\nHost: \tfoo.com\t \r\nCookie: \t \r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.header("Host").unwrap(), "foo.com");
        assert_eq!(req.header("Cookie").unwrap(), "");
    }
}

test! {
    // test the scalar parsing
    test_request_header_value_htab_short,
    b"GET / HTTP/1.1\r\nUser-Agent: some\tagent\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.header("User-Agent").unwrap(), "some\tagent");
    }
}

test! {
    // test the sse42 parsing
    test_request_header_value_htab_med,
    b"GET / HTTP/1.1\r\nUser-Agent: 1234567890some\tagent\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.headers().len(), 1);
        assert_eq!(req.header("User-Agent").unwrap(), "1234567890some\tagent");
    }
}

test! {
    // test the avx2 parsing
    test_request_header_value_htab_long,
    b"GET / HTTP/1.1\r\nUser-Agent: 1234567890some\t1234567890agent1234567890\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.headers().len(), 1);
        assert_eq!(req.header("User-Agent").unwrap(), "1234567890some\t1234567890agent1234567890");
    }
}

// test! {
//     test_request_headers_max,
//     b"GET / HTTP/1.1\r\nA: A\r\nB: B\r\nC: C\r\nD: D\r\n\r\n",
//     |req| {
//         assert_eq!(req.headers().len(), NUM_OF_HEADERS);
//     }
// }

test! {
    test_request_multibyte,
    b"GET / HTTP/1.1\r\nHost: foo.com\r\nUser-Agent: \xe3\x81\xb2\xe3/1.0\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.header("Host").unwrap(), "foo.com");
        assert_eq!(req.header("User-Agent").unwrap(), "?");
    }
}

test! {
    test_request_newlines,
    b"GET / HTTP/1.1\nHost: foo.bar\n\n",
    |req| {
        assert_eq!(req.path(), "/");
        assert_eq!(req.method(), "GET");
        assert_eq!(req.header("Host").unwrap(), "foo.bar");
    }
}

test! {
    test_request_empty_lines_prefix,
    b"\r\n\r\nGET / HTTP/1.1\r\n\r\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_empty_lines_prefix_lf_only,
    b"\n\nGET / HTTP/1.1\n\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_path_backslash,
    b"\n\nGET /\\?wayne\\=5 HTTP/1.1\n\n",
    |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.full_path(), "/\\?wayne\\=5");
        assert_eq!(req.headers().len(), 0);
    }
}

test! {
    test_request_with_invalid_token_delimiter,
    b"GET\n/ HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
    Error::ParseError
}

test! {
    test_request_with_invalid_but_short_version,
    b"GET / HTTP/1!",
    Error::ParseVersion
}

#[test]
fn test_request_partial() {
    match parse(b"GET / HTTP/1.1\r\n\r".to_vec()) {
        Err(Error::ConnectionClosed) => {}
        _ => panic!(),
    }
}

#[test]
fn test_request_partial_version() {
    match parse(b"GET / HTTP/1.".to_vec()) {
        Err(Error::ConnectionClosed) => {}
        _ => panic!(),
    }
}
