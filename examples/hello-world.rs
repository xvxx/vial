use std::fmt;

#[derive(Debug)]
struct Request {
    url: String,
    method: String,
}

impl Request {
    fn method(&self) -> &str {
        &self.method
    }

    fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Default, Debug)]
struct Response {
    body: String,
}

impl Response {
    fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl<T: AsRef<str>> From<T> for Response {
    fn from(s: T) -> Response {
        Response {
            body: s.as_ref().to_string(),
        }
    }
}

#[allow(unused)]
enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

macro_rules! vial {
    ( $($method:ident $path:expr => $body:expr;)* ) => {
        fn route(req: Request) -> Response {
            match (req.method(), req.url()) {
                $( (stringify!($method), $path) => $body(req), )*
                _ => Response::from("404 Not Found"),
            }
        }
    };
}

vial! {
    POST "/blah" => |_| "Dingo!".into();

    GET "/hello/world" => |_| "Hiya".into();

    GET "/info" => |req| {
        format!("<h1>Request Information:</h1><pre>{:?}</p>", req).into()
    };

    GET "/" => |_| {
        // sinatra-like, i guess
        "Cool".into()
    };
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "/".into());
    let method = args.next().unwrap_or_else(|| "GET".into()).to_uppercase();
    let req = Request {
        method: method,
        url: path,
    };
    println!("{}", route(req));
}
