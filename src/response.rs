use {
    crate::{asset, util},
    std::{
        collections::HashMap,
        fmt, fs,
        io::{self, Read},
        path::Path,
    },
};

pub struct Response {
    pub code: usize,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub buf: Vec<u8>,
    pub content_type: String,
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("code", &self.code)
            .field("content_type", &self.content_type)
            .field("body", &self.body)
            .finish()
    }
}

impl Default for Response {
    fn default() -> Response {
        Response {
            code: 200,
            body: String::new(),
            buf: Vec::new(),
            headers: HashMap::new(),
            content_type: "text/html; charset=utf8".to_string(),
        }
    }
}

impl Response {
    pub fn new() -> Response {
        Response::default()
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }

    pub fn from_file(path: &str) -> Response {
        Response::default().with_file(path)
    }

    pub fn with_code(mut self, code: usize) -> Response {
        self.code = code;
        self
    }

    pub fn with_body(mut self, body: &str) -> Response {
        self.body.clear();
        self.body.push_str(body);
        self
    }

    pub fn with_file(mut self, path: &str) -> Response {
        match fs::File::open(asset::normalize_path(path)) {
            Ok(mut file) => {
                self.content_type.clear();
                self.content_type.push_str(util::content_type(path));
                file.read_to_end(&mut self.buf);
                self
            }

            Err(e) => self
                .with_body(&format!("<h1>500 Internal Error</h1><pre>{:?}", e))
                .with_code(500),
        }
    }

    pub fn len(&self) -> usize {
        if self.buf.is_empty() {
            self.body.len()
        } else {
            self.buf.len()
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.buf.is_empty() {
            write!(f, "{}", String::from_utf8_lossy(&self.buf))
        } else {
            write!(f, "{}", self.body)
        }
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

impl From<std::borrow::Cow<'_, [u8]>> for Response {
    fn from(i: std::borrow::Cow<'_, [u8]>) -> Response {
        Response {
            body: String::from_utf8_lossy(&i).to_string(),
            ..Response::default()
        }
    }
}
