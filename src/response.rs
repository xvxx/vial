use std::io::Write;

#[cfg(feature = "compression")] //REMOVE
use libflate::gzip::Encoder;

use crate::Compression;
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
            Body::None => write!(f, "None"),
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
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_lowercase(),
            "text/html; charset=utf8".into(),
        );
        headers.insert("Content-Length".to_lowercase(), "0".into());

        Self {
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// HTTP Status Code
    #[must_use]
    pub fn code(&self) -> usize {
        self.code
    }

    /// Either the inferred or user-set Content-Type for this Response.
    #[must_use]
    pub fn content_type(&self) -> &str {
        self.header("Content-Type").unwrap_or("")
    }

    /// Response body.
    #[must_use]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    /// Take a peek at all the headers for this response.
    #[must_use]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get an individual header. `name` is case insensitive.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(&name.to_lowercase())
            .map(std::convert::AsRef::as_ref)
    }

    /// Set an individual header.
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.to_string());
    }

    #[cfg(feature = "cookies")]
    /// Get an individual cookie. `name` is case insensitive.
    #[must_use]
    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.cookies
            .get(&name.to_lowercase())
            .map(std::convert::AsRef::as_ref)
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
    pub fn from<T: Into<Self>>(from: T) -> Self {
        from.into()
    }

    /// Create a response from an asset. See the
    /// [`asset`](asset/index.html) module for more information on
    /// using assets.
    #[must_use]
    pub fn from_asset(path: &str) -> Self {
        Self::default().with_asset(path)
    }

    /// Create a response from a (boxed) `io::Read`.
    #[must_use]
    pub fn from_reader(reader: Box<dyn io::Read>) -> Self {
        Self::default().with_reader(reader)
    }

    /// Creates a response from a file on disk.
    /// TODO: Path?
    #[must_use]
    pub fn from_file(path: &str) -> Self {
        Self::default().with_file(path)
    }

    /// Creates a 500 response from an error, displaying it.
    pub fn from_error<E: error::Error>(err: E) -> Self {
        Self::default().with_error(err)
    }

    /// Creates a new Response and sets the given header, in
    /// addition to the defaults.
    #[must_use]
    pub fn from_header(name: &str, value: &str) -> Self {
        Self::default().with_header(name, value)
    }

    #[cfg(feature = "cookies")]
    /// Creates a new Response and sets the given cookie, in
    /// addition to the defaults.
    #[must_use]
    pub fn from_cookie(name: &str, value: &str) -> Self {
        Self::default().with_cookie(name, value)
    }

    /// Creates a new default Response with the given body.
    pub fn from_body<S: AsRef<str>>(body: S) -> Self {
        Self::default().with_body(body)
    }

    /// Creates a new `text/plain` Response with the given body.
    pub fn from_text<S: AsRef<str>>(text: S) -> Self {
        Self::default().with_text(text)
    }

    /// Creates a new response with the given HTTP Status Code.
    #[must_use]
    pub fn from_code(code: usize) -> Self {
        Self::default().with_code(code)
    }

    /// Creates a new response with the given HTTP Status Code.
    #[must_use]
    pub fn with_code(mut self, code: usize) -> Self {
        self.code = code;
        match code {
            404 => self.with_body("404 Not Found"),
            500 => self.with_body("500 Internal Server Error"),
            _ => self,
        }
    }

    /// Body builder. Returns a Response with the given body.
    pub fn with_body<S: AsRef<str>>(mut self, body: S) -> Self {
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
    pub fn with_json<T: serde::Serialize>(self, value: T) -> Self {
        // Panics if to_value returns Err because this probably indicates a programming error.
        self.with_body(
            serde_json::to_value(value)
                .expect("Serialize failed")
                .to_string(),
        )
        .with_header("Content-Type", "application/json")
    }

    /// Returns a `text/plain` Response with the given body.
    pub fn with_text<S: AsRef<str>>(self, text: S) -> Self {
        self.with_body(text)
            .with_header("Content-Type", "text/plain; charset=utf8")
    }

    /// Returns a Response using the given reader for the body.
    #[must_use]
    pub fn with_reader(mut self, reader: Box<dyn io::Read>) -> Self {
        self.body = Body::Reader(reader);
        self
    }

    /// Uses an asset for the given body and sets the `Content-Type`
    /// header based on the file's extension.
    ///
    /// See the [`asset`](asset/index.html) module for more
    /// information on using assets.
    #[must_use]
    pub fn with_asset(mut self, path: &str) -> Self {
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
    #[must_use]
    pub fn with_file(mut self, path: &str) -> Self {
        if !std::path::Path::new(path).exists() {
            return Self::from(404);
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
    pub fn with_error<E: error::Error>(self, err: E) -> Self {
        self.with_code(500)
            .with_body(&format!("<h1>500 Internal Error</h1><pre>{:?}", err))
    }

    /// Returns a Response with the given header set to the value.
    #[must_use]
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.set_header(key, value);
        self
    }

    #[cfg(feature = "cookies")]
    /// Returns a Response with the given cookie set to the value.
    #[must_use]
    pub fn with_cookie(mut self, key: &str, value: &str) -> Self {
        self.set_cookie(key, value);
        self
    }

    #[cfg(feature = "cookies")]
    /// Returns a Response with an instruction to remove the cookie.
    #[must_use]
    pub fn without_cookie(mut self, key: &str) -> Self {
        self.remove_cookie(key);
        self
    }

    /// Length of the body.
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.body {
            Body::String(s) => s.len(),
            Body::Reader(..) => self
                .header("Content-Length")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            Body::None => 0,
        }
    }

    /// Is ths response empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a 302 redirect to the given URL.
    pub fn redirect_to<U: AsRef<str>>(url: U) -> Self {
        Self::from(302).with_header("location", url.as_ref())
    }

    /// Writes this response to a stream.
    /// # Errors
    /// This function errors if at some point the reader failed to be copied out of
    pub fn write<W: Write>(mut self, mut w: W, compression: &Option<Compression>) -> Result<()> {
        // gross - move into print_headers or something
        let mut default_headers = format!(
            "HTTP/1.1 {} OK\r\nServer: ~ vial {} ~\r\nDate: {}\r\nConnection: close\r\n",
            self.code,
            crate::VERSION,
            util::http_current_date(),
        );
        #[cfg(not(feature = "compression"))]
        if compression.is_some() {
            println!("Compression option given, but compression feature disabled!");
        }
        let mut body = vec![];
        // let mut len;
        match self.body {
            Body::Reader(mut reader) => {
                #[cfg(feature = "compression")]
                {
                    match compression {
                        Some(compression) => match compression {
                            Compression::Gzip => {
                                let mut vec = vec![];
                                if reader.read_to_end(&mut vec).is_ok() {
                                    body.write_all(
                                        &libflate::gzip::Encoder::new(vec)
                                            .expect("Failed to initiate gzip encoder")
                                            .finish()
                                            .into_result()
                                            .expect("Failed to compress with gzip"),
                                    )?;
                                }
                            }
                            Compression::Deflate => {
                                let mut vec = vec![];
                                if reader.read_to_end(&mut vec).is_ok() {
                                    body.write_all(
                                        &libflate::deflate::Encoder::new(vec)
                                            .finish()
                                            .into_result()
                                            .expect("Failed to compress with deflate"),
                                    )?;
                                }
                            }
                            Compression::Brotli => {
                                io::copy(
                                    &mut brotli2::read::BrotliEncoder::new(reader, 6),
                                    &mut body,
                                )?;
                            }
                            Compression::Zstd => {
                                let mut vec = vec![];
                                if reader.read_to_end(&mut vec).is_ok() {
                                    let zstd = zstd::stream::write::Encoder::new(vec, 3)
                                        .expect("Failed to initialize zstd encoder")
                                        .finish()
                                        .expect("Failed to compress with zstd");
                                    body.write_all(&zstd)?;
                                }
                            }
                            Compression::Identity => {
                                io::copy(&mut reader, &mut body)?;
                            }
                        },
                        None => {
                            io::copy(&mut reader, &mut body)?;
                        }
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
                    match compression {
                        Some(compression) => match compression {
                            Compression::Gzip => {
                                let mut encoder =
                                    Encoder::new(vec![]).expect("Failed to create gzip encoder");
                                encoder.write_all(s.as_bytes())?;
                                body.write_all(
                                    &encoder
                                        .finish()
                                        .into_result()
                                        .expect("Failed to compress gzip"),
                                )?;
                            }
                            Compression::Deflate => {
                                let mut vec = vec![];
                                if vec.write_all(s.as_bytes()).is_ok() {
                                    body.write_all(
                                        &libflate::deflate::Encoder::new(vec)
                                            .finish()
                                            .into_result()
                                            .expect("Failed to compress with deflate"),
                                    )?;
                                }
                            }
                            Compression::Brotli => {
                                let mut vec = vec![];
                                if vec.write_all(s.as_bytes()).is_ok() {
                                    let x = brotli2::read::BrotliEncoder::new(
                                        std::io::Cursor::new(vec),
                                        6,
                                    );
                                    io::copy(&mut x.into_inner(), &mut body)?;
                                }
                            }
                            Compression::Zstd => {
                                let mut vec = vec![];
                                if vec.write_all(s.as_bytes()).is_ok() {
                                    body.write_all(
                                        &zstd::stream::write::Encoder::new(vec, 3)
                                            .expect("Failed to initiate zstd encoder")
                                            .finish()?,
                                    )?;
                                }
                            }
                            Compression::Identity => {
                                body.write_all(s.as_bytes())?;
                            }
                        },
                        None => {
                            body.write_all(s.as_bytes())?;
                        }
                    }
                }

                #[cfg(not(feature = "compression"))]
                {
                    body.write_all(s.as_bytes())?;
                }
            }
            Body::None => {}
        }
        self.headers
            .insert("content-length".to_lowercase(), body.len().to_string());
        #[cfg(feature = "compression")]
        {
            if let Some(compression) = compression {
                if compression != &Compression::Identity {
                    self.headers.insert(
                        "content-encoding".to_lowercase(),
                        match compression {
                            Compression::Gzip => "gzip",
                            Compression::Deflate => "deflate",
                            Compression::Brotli => "br",
                            Compression::Zstd => "zstd",
                            Compression::Identity => "", // this branch can't be reached
                        }
                        .to_string(),
                    );
                }
            }
        }

        default_headers.push_str(
            &self
                .headers
                .iter()
                .map(|(key, val)| format!("{}: {}", key, val))
                .collect::<Vec<_>>()
                .join("\r\n"),
        );

        if !default_headers.ends_with("\r\n") {
            default_headers.push_str("\r\n");
        }

        #[cfg(feature = "cookies")]
        {
            for (name, val) in self.cookies {
                default_headers.push_str("Set-Cookie: ");
                default_headers.push_str(&name);
                default_headers.push('=');
                if val.is_empty() {
                    default_headers.push_str("; Expires=Thu, 01 Jan 1970 00:00:00 GMT");
                } else {
                    default_headers.push_str(&val);
                }
                default_headers.push_str("\r\n");
            }
        }

        default_headers.push_str("\r\n");
        w.write_all(default_headers.as_bytes())?;
        w.write_all(&body)?;
        w.flush()?;
        Ok(())
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        Self::from_body(s.to_string())
    }
}

impl From<&String> for Response {
    fn from(s: &String) -> Self {
        Self::from_body(s.clone())
    }
}

impl From<String> for Response {
    fn from(body: String) -> Self {
        Self::from_body(body)
    }
}

impl From<usize> for Response {
    fn from(i: usize) -> Self {
        Self::from_code(i)
    }
}

impl From<std::borrow::Cow<'_, [u8]>> for Response {
    fn from(i: std::borrow::Cow<'_, [u8]>) -> Self {
        Self::from_body(String::from_utf8_lossy(&i).to_string())
    }
}
