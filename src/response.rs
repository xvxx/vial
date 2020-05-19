use std::{fmt, fs, path::Path};

#[derive(Debug)]
pub struct Response {
    code: usize,
    body: String,
}

impl Default for Response {
    fn default() -> Response {
        Response {
            code: 200,
            body: String::new(),
        }
    }
}

impl Response {
    pub fn new() -> Response {
        Response::default()
    }

    pub fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }

    pub fn with_code(mut self, code: usize) -> Response {
        self.code = code;
        self
    }

    pub fn with_body(mut self, body: &str) -> Response {
        self.body = body.to_string();
        self
    }

    pub fn from_file<P: AsRef<Path>>(mut self, path: P) -> Response {
        match fs::read_to_string(path) {
            Ok(body) => self.body = body,
            Err(e) => {
                self.body = format!("<h1>500 Internal Error</h1><pre>{:?}", e);
                self.code = 500;
            }
        }
        self
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Response {
        Response {
            body: s.to_string(),
            ..Response::default()
        }
    }
}

impl From<&String> for Response {
    fn from(s: &String) -> Response {
        Response {
            body: s.clone(),
            ..Response::default()
        }
    }
}

impl From<String> for Response {
    fn from(body: String) -> Response {
        Response {
            body,
            ..Response::default()
        }
    }
}

impl From<usize> for Response {
    fn from(i: usize) -> Response {
        Response {
            code: i.into(),
            ..Response::default()
        }
    }
}
