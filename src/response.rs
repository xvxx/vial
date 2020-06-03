use {
    crate::{asset, util, Result},
    std::{
        collections::HashMap,
        error, fmt, fs,
        io::{self, BufReader},
    },
};

pub struct Response {
    pub code: usize,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub buf: Vec<u8>,
    pub reader: Box<dyn io::Read>,
    /// TODO: hax
    pub is_reader: bool,
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
            reader: Box::new(io::empty()),
            is_reader: false,
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

    pub fn header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }

    pub fn from_asset(path: &str) -> Response {
        Response::default().with_asset(path)
    }

    pub fn from_reader(reader: Box<dyn io::Read>) -> Response {
        Response::default().with_reader(reader)
    }

    pub fn from_file(path: &str) -> Response {
        Response::default().with_file(path)
    }

    pub fn from_error<E: error::Error>(err: E) -> Response {
        Response::from(500).with_error(err)
    }

    pub fn from_header(name: &str, value: &str) -> Response {
        Response::default().with_header(name, value)
    }

    pub fn from_body<S: AsRef<str>>(body: S) -> Response {
        Response::default().with_body(body)
    }

    pub fn from_text<S: AsRef<str>>(text: S) -> Response {
        Response::default().with_text(text)
    }

    pub fn with_code(mut self, code: usize) -> Response {
        self.code = code;
        self
    }

    pub fn with_body<S: AsRef<str>>(mut self, body: S) -> Response {
        self.body.clear();
        self.body.push_str(body.as_ref());
        self
    }

    pub fn with_text<S: AsRef<str>>(mut self, text: S) -> Response {
        self.with_body(text)
            .with_header("Content-Type", "text/plain")
    }

    pub fn with_reader(mut self, reader: Box<dyn io::Read>) -> Response {
        self.reader = reader;
        self.is_reader = true;
        self
    }

    pub fn with_asset(mut self, path: &str) -> Response {
        if let Some(path) = asset::normalize_path(path) {
            if asset::exists(&path) {
                if asset::is_bundled() {
                    if let Some(reader) = asset::as_reader(&path) {
                        self.header("ETag", asset::etag(&path).as_ref());
                        self.content_type = util::content_type(&path).to_string();
                        return self.with_reader(reader);
                    }
                } else {
                    return self.with_file(&path);
                }
            }
        }
        self.with_code(404)
    }

    pub fn with_file(mut self, path: &str) -> Response {
        match fs::File::open(path) {
            Ok(mut file) => {
                self.header("ETag", &asset::etag(path).as_ref());
                self.content_type.clear();
                self.content_type.push_str(util::content_type(path));
                self.with_reader(Box::new(BufReader::new(file)))
            }

            Err(e) => self.with_error(Box::new(e)),
        }
    }

    pub fn with_error<E: error::Error>(mut self, err: E) -> Response {
        self.with_code(500)
            .with_body(&format!("<h1>500 Internal Error</h1><pre>{}", err))
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Response {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn len(&self) -> usize {
        if self.is_reader {
            0
        } else if self.buf.is_empty() {
            self.body.len()
        } else {
            self.buf.len()
        }
    }

    pub fn redirect_to<U: AsRef<str>>(url: U) -> Response {
        Response::from(302).with_header("location", url.as_ref())
    }

    pub fn write<W: io::Write>(mut self, mut w: W) -> Result<()> {
        // we don't set Content-Length on static files we stream
        let content_length = if self.len() > 0 {
            format!("Content-Length: {}\r\n", self.len())
        } else {
            "".to_string()
        };

        // gross - move into print_headers or something
        let mut header = format!(
            "HTTP/1.1 {} OK\r\nServer: ~ vial {} ~\r\nDate: {}\r\nContent-Type: {}\r\n{}Connection: close\r\n",
            self.code, env!("CARGO_PKG_VERSION"), util::http_current_date(),
            self.content_type, content_length,
        );

        // TODO check for content-type, date, etc
        header.push_str(
            &self
                .headers
                .iter()
                .map(|(key, val)| format!("{}: {}", key, val))
                .collect::<Vec<_>>()
                .join("\r\n"),
        );

        if !header.ends_with("\r\n") {
            header.push_str("\r\n");
        }
        header.push_str("\r\n");

        w.write(header.as_bytes())?;

        if self.is_reader {
            io::copy(&mut self.reader, &mut w);
        } else if self.buf.is_empty() {
            w.write(self.body.as_bytes())?;
        } else {
            w.write(&self.buf)?;
        }

        w.flush()?;

        Ok(())
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
            body: match i {
                404 => "404 Not Found".into(),
                500 => "500 Internal Server Error".into(),
                _ => "".into(),
            },
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
