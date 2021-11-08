use std::io::Write;

#[cfg(feature = "compression")] //REMOVE
use libflate::gzip::Encoder;
use {
    crate::{asset, util, Result},
    std::{
        collections::HashMap,
        error, fmt, fs,
        io::{self, BufReader},
    },
};

/// Response Body. Will be either a `String` or `io::Read`, like from
/// a File.
enum Body {
    None,
    String(String),
    Reader(Box<dyn io::Read>),
}

impl Body {
    /// Body as a string. Always empty if this is a `Reader`,
    /// otherwise we'd have to consume the stream.
    fn as_str(&self) -> &str {
        if let Body::String(s) = self {
            s.as_ref()
        } else {
            ""
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::String(s) => write!(f, "{}", s),
            Body::Reader(..) => write!(f, "(io::Read)"),
            _ => write!(f, "None"),
        }
    }
}

/// Each request ultimately ends in a `Response` that is served to the
/// client and then discarded, like fallen flower petals. Together
/// with [`Request`](struct.request.html) and
/// [`Responder`](trait.Responder.html) it forms the holy trinity of
/// `R`'s in Vial.
///
/// Rather than use the "Builder" pattern like more mature and better
/// designed libraries, Vial's `Response` lets you set properties
/// either directly or using Builder-style methods:
///
/// ```no_run
/// vial::routes! {
///     GET "/404" => |_| Response::from(404)
///         .with_header("Content-Type", "text/plain")
///         .with_body("404 Not Found");
/// }
/// ```
///
/// It also defaults to `text/html`, so you need to use
/// [`with_header()`](#method.with_header) or
/// [`header()`](#method.header) to send plain text.
pub struct Response {
    /// HTTP Status Code
    code: usize,

    /// The headers we're sending back.
    headers: HashMap<String, String>,

    /// Response body.
    body: Body,

    #[cfg(feature = "cookies")]
    /// Cookies to set.
    cookies: HashMap<String, String>,
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.headers == other.headers
            && self.content_type() == other.content_type()
            && self.body.as_str() == other.body.as_str()
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("code", &self.code)
            .field("content_type", &self.content_type())
            .field("body", &self.body.as_str())
            .finish()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl Default for Response {
    fn default() -> Response {
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_lowercase(),
            "text/html; charset=utf8".into(),
        );
        headers.insert("Content-Length".to_lowercase(), "0".into());

        Response {
            code: 200,
            body: Body::None,
            headers,

            #[cfg(feature = "cookies")]
            cookies: HashMap::new(),
        }
    }
}

impl Response {
    /// Create a new, empty, 200 response - ready for HTML!
    pub fn new() -> Response {
        Response::default()
    }

    /// HTTP Status Code
    pub fn code(&self) -> usize {
        self.code
    }

    /// Either the inferred or user-set Content-Type for this Response.
    pub fn content_type(&self) -> &str {
        self.header("Content-Type").unwrap_or("")
    }

    /// Response body.
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    /// Take a peek at all the headers for this response.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get an individual header. `name` is case insensitive.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(|h| h.as_ref())
    }

    /// Set an individual header.
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.to_string());
    }

    #[cfg(feature = "cookies")]
    /// Get an individual cookie. `name` is case insensitive.
    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.cookies.get(&name.to_lowercase()).map(|h| h.as_ref())
    }

    #[cfg(feature = "cookies")]
    /// Set a cookie.
    pub fn set_cookie(&mut self, name: &str, value: &str) {
        self.cookies.insert(name.to_lowercase(), value.into());
    }

    #[cfg(feature = "cookies")]
    /// Remove a cookie from the client.
    pub fn remove_cookie(&mut self, name: &str) {
        self.cookies.insert(name.to_lowercase(), "".into());
    }

    /// Convert into a Response.
    pub fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }

    /// Create a response from an asset. See the
    /// [`asset`](asset/index.html) module for more information on
    /// using assets.
    pub fn from_asset(path: &str) -> Response {
        Response::default().with_asset(path)
    }

    /// Create a response from a (boxed) io::Read.
    pub fn from_reader(reader: Box<dyn io::Read>) -> Response {
        Response::default().with_reader(reader)
    }

    /// Creates a response from a file on disk.
    /// TODO: Path?
    pub fn from_file(path: &str) -> Response {
        Response::default().with_file(path)
    }

    /// Creates a 500 response from an error, displaying it.
    pub fn from_error<E: error::Error>(err: E) -> Response {
        Response::default().with_error(err)
    }

    /// Creates a new Response and sets the given header, in
    /// addition to the defaults.
    pub fn from_header(name: &str, value: &str) -> Response {
        Response::default().with_header(name, value)
    }

    #[cfg(feature = "cookies")]
    /// Creates a new Response and sets the given cookie, in
    /// addition to the defaults.
    pub fn from_cookie(name: &str, value: &str) -> Response {
        Response::default().with_cookie(name, value)
    }

    /// Creates a new default Response with the given body.
    pub fn from_body<S: AsRef<str>>(body: S) -> Response {
        Response::default().with_body(body)
    }

    /// Creates a new `text/plain` Response with the given body.
    pub fn from_text<S: AsRef<str>>(text: S) -> Response {
        Response::default().with_text(text)
    }

    /// Creates a new response with the given HTTP Status Code.
    pub fn from_code(code: usize) -> Response {
        Response::default().with_code(code)
    }

    /// Creates a new response with the given HTTP Status Code.
    pub fn with_code(mut self, code: usize) -> Response {
        self.code = code;
        match code {
            404 => self.with_body("404 Not Found"),
            500 => self.with_body("500 Internal Server Error"),
            _ => self,
        }
    }

    /// Body builder. Returns a Response with the given body.
    pub fn with_body<S: AsRef<str>>(mut self, body: S) -> Response {
        let body = body.as_ref();
        self.body = Body::String(body.to_string());
        self.set_header("Content-Length", &body.len().to_string());
        self
    }

    /// Returns an `application/json` Response with a body serialized as JSON
    /// from the given value.
    ///
    /// The `json_serde` feature must be enabled in `Cargo.toml`.
    #[cfg(feature = "json_serde")]
    pub fn with_json<T: serde::Serialize>(self, value: T) -> Response {
        // Panics if to_value returns Err because this probably indicates a programming error.
        self.with_body(
            serde_json::to_value(value)
                .expect("Serialize failed")
                .to_string(),
        )
        .with_header("Content-Type", "application/json")
    }

    /// Returns a `text/plain` Response with the given body.
    pub fn with_text<S: AsRef<str>>(self, text: S) -> Response {
        self.with_body(text)
            .with_header("Content-Type", "text/plain; charset=utf8")
    }

    /// Returns a Response using the given reader for the body.
    pub fn with_reader(mut self, reader: Box<dyn io::Read>) -> Response {
        self.body = Body::Reader(reader);
        self
    }

    /// Uses an asset for the given body and sets the `Content-Type`
    /// header based on the file's extension.
    ///
    /// See the [`asset`](asset/index.html) module for more
    /// information on using assets.
    pub fn with_asset(mut self, path: &str) -> Response {
        if let Some(path) = asset::normalize_path(path) {
            if asset::exists(&path) {
                if asset::is_bundled() {
                    if let Some(reader) = asset::as_reader(&path) {
                        self.set_header("ETag", asset::etag(&path).as_ref());
                        self.set_header("Content-Type", util::content_type(&path));
                        self.set_header("Content-Length", &asset::size(&path).to_string());
                        return self.with_reader(reader);
                    }
                } else {
                    return self.with_file(&path);
                }
            }
        } else if asset::exists(path) {
            if asset::is_bundled() {
                if let Some(reader) = asset::as_reader(path) {
                    self.set_header("ETag", asset::etag(path).as_ref());
                    self.set_header("Content-Type", util::content_type(path));
                    self.set_header("Content-Length", &asset::size(path).to_string());
                    return self.with_reader(reader);
                }
            } else {
                return self.with_file(path);
            }
        }
        self.with_code(404)
    }

    /// Sets this Response's body to the body of the given file and
    /// sets the `Content-Type` header based on the file's extension.
    pub fn with_file(mut self, path: &str) -> Response {
        if !std::path::Path::new(path).exists() {
            return Response::from(404);
        }
        match fs::File::open(path) {
            Ok(file) => {
                self.set_header("ETag", asset::etag(path).as_ref());
                self.set_header("Content-Type", util::content_type(path));
                self.set_header("Content-Length", &util::file_size(path).to_string());
                self.with_reader(Box::new(BufReader::new(file)))
            }

            Err(e) => self.with_error(Box::new(e)),
        }
    }

    /// Sets the response code to 500 and the body to the error's text.
    pub fn with_error<E: error::Error>(self, err: E) -> Response {
        self.with_code(500)
            .with_body(&format!("<h1>500 Internal Error</h1><pre>{:?}", err))
    }

    /// Returns a Response with the given header set to the value.
    pub fn with_header(mut self, key: &str, value: &str) -> Response {
        self.set_header(key, value);
        self
    }

    #[cfg(feature = "cookies")]
    /// Returns a Response with the given cookie set to the value.
    pub fn with_cookie(mut self, key: &str, value: &str) -> Response {
        self.set_cookie(key, value);
        self
    }

    #[cfg(feature = "cookies")]
    /// Returns a Response with an instruction to remove the cookie.
    pub fn without_cookie(mut self, key: &str) -> Response {
        self.remove_cookie(key);
        self
    }

    /// Length of the body.
    pub fn len(&self) -> usize {
        match &self.body {
            Body::String(s) => s.len(),
            Body::Reader(..) => self
                .header("Content-Length")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            _ => 0,
        }
    }

    /// Is ths response empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a 302 redirect to the given URL.
    pub fn redirect_to<U: AsRef<str>>(url: U) -> Response {
        Response::from(302).with_header("location", url.as_ref())
    }

    /// Writes this response to a stream.
    pub fn write<W: io::Write>(mut self, mut w: W, _gzip: bool) -> Result<()> {
        // gross - move into print_headers or something
        let mut header = format!(
            "HTTP/1.1 {} OK\r\nServer: ~ vial {} ~\r\nDate: {}\r\nConnection: close\r\n",
            self.code,
            crate::VERSION,
            util::http_current_date(),
        );

        let mut body = vec![];
        // let mut len;
        match self.body {
            Body::Reader(mut reader) => {
                #[cfg(feature = "compression")]
                {
                    if _gzip {
                        let mut vec = vec![];
                        if reader.read_to_end(&mut vec).is_ok() {
                            body.write_all(
                                &Encoder::new(vec).unwrap().finish().into_result().unwrap(),
                            )?;
                        }
                    } else {
                        io::copy(&mut reader, &mut body)?;
                    }
                }

                #[cfg(not(feature = "compression"))]
                {
                    io::copy(&mut reader, &mut body)?;
                }
            }
            Body::String(s) => {
                #[cfg(feature = "compression")]
                {
                    if _gzip {
                        let mut encoder = Encoder::new(vec![]).unwrap();
                        encoder.write_all(s.as_bytes())?;
                        body.write_all(&encoder.finish().into_result().unwrap())?;
                    } else {
                        body.write_all(s.as_bytes())?;
                    }
                }

                #[cfg(not(feature = "compression"))]
                body.write_all(s.as_bytes())?;
            }
            _ => {}
        }
        self.headers
            .insert("content-length".to_lowercase(), body.len().to_string());
        #[cfg(feature = "compression")]
        if _gzip {
            self.headers
                .insert("content-encoding".to_lowercase(), "gzip".into());
        }

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

        #[cfg(feature = "cookies")]
        {
            for (name, val) in self.cookies {
                header.push_str("Set-Cookie: ");
                header.push_str(&name);
                header.push('=');
                if val.is_empty() {
                    header.push_str("; Expires=Thu, 01 Jan 1970 00:00:00 GMT");
                } else {
                    header.push_str(&val);
                }
                header.push_str("\r\n");
            }
        }

        header.push_str("\r\n");
        w.write_all(header.as_bytes())?;
        w.write_all(&body)?;
        w.flush()?;
        Ok(())
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Response {
        Response::from_body(s.to_string())
    }
}

impl From<&String> for Response {
    fn from(s: &String) -> Response {
        Response::from_body(s.clone())
    }
}

impl From<String> for Response {
    fn from(body: String) -> Response {
        Response::from_body(body)
    }
}

impl From<usize> for Response {
    fn from(i: usize) -> Response {
        Response::from_code(i)
    }
}

impl From<std::borrow::Cow<'_, [u8]>> for Response {
    fn from(i: std::borrow::Cow<'_, [u8]>) -> Response {
        Response::from_body(String::from_utf8_lossy(&i).to_string())
    }
}
