use crate::{http_parser, util, Result, TypeCache};
use std::{collections::HashMap, fmt, io::Read, mem, net::TcpStream, rc::Rc, str};

/// A `(start, end)` tuple representing a the location of some part of
/// a Request in a raw buffer, such as the requested URL's path.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span(pub usize, pub usize);

impl Span {
    /// Find and return the str this span represents from the given
    /// buffer, which should be the raw HTTP request.
    pub fn from_buf<'buf>(&self, buf: &'buf [u8]) -> &'buf str {
        if self.1 >= self.0 && self.1 <= buf.len() {
            str::from_utf8(&buf[self.0..self.1]).unwrap_or("?")
        } else {
            ""
        }
    }
}
/// Request contains all the info about a client's request. It's
/// handed to your actions and filters, and is dropped after
/// responding to the client.
///
/// The main ways you'll be using Request are:
///
/// - **[arg(&str)](#method.arg)**: Getting arguments from routes that
///   include parameters, such as `GET "/:page.md" => show;`. You'd
///   use `request.arg("page")` in this case.
/// - **[query(&str)](#method.query)**: Getting decoded query
///   parameters. If a request includes `?id=5&loc=CA` you can use
///   `request.query("id")` and `request.query("loc")` to get both
///   values.
/// - **[form(&str)](#method.form)**: Same as above, but with
///   submitted `<form>` data. Make sure your `<input>` elements have
///   the right `name=` attribute.
/// - **[path()](#method.path)**: The path requested, starting with an
///   `/`, not including any `?query`.
/// - **[full_path()](#method.full_path)**: The full path starting
///   with `/`, including `?query`.
/// - **[method()](#method.method)**: If you need the HTTP method.
/// - **[cache()](#method.cache)**: The request local cache.
///
/// You may also modify a Request in a `filter` using:
///
/// - **[set_arg(&str, &str)](#method.set_arg)**
/// - **[set_form(&str, &str)](#method.set_form)**
/// - **[set_query(&str, &str)](#method.set_query)**
/// - **[set_path(&str, &str)](#method.set_path)**
/// - **[set_method(&str, &str)](#method.set_method)**
/// - **[set_body(&str, &str)](#method.set_body)**
pub struct Request {
    /// The raw request.
    buffer: Vec<u8>,

    /// These all reference `buffer`.
    /// Path starts with `/` and doesn't include `?query`.
    path: Span,
    /// Same as `path` but includes `?query`.
    full_path: Span,
    method: Span,
    body: Span,
    headers: Vec<(Span, Span)>,

    /// Maps of form and URL args, percent decoded.
    args: HashMap<String, String>,
    form: HashMap<String, String>,

    /// Local request cache.
    cache: Rc<TypeCache>,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers = HashMap::new();
        for (k, v) in &self.headers {
            headers.insert(k.from_buf(&self.buffer), v.from_buf(&self.buffer));
        }
        f.debug_struct("Request")
            .field("method", &self.method.from_buf(&self.buffer))
            .field("path", &self.path.from_buf(&self.buffer))
            .field("full_path", &self.full_path.from_buf(&self.buffer))
            .field("headers", &headers)
            .finish()
    }
}

impl Request {
    /// Create a new Request from a raw one. You probably want
    /// `default()` to get an empty `Request`.
    pub fn new(
        method: Span,
        full_path: Span,
        path: Span,
        headers: Vec<(Span, Span)>,
        body: Span,
        buffer: Vec<u8>,
    ) -> Request {
        Request {
            method,
            full_path,
            path,
            headers,
            body,
            buffer,
            ..Request::default()
        }
    }
    /// Produce an empty Request.
    pub fn default() -> Request {
        Request {
            path: Span(0, 0),
            full_path: Span(0, 0),
            method: Span(0, 0),
            body: Span(0, 0),
            headers: Vec::new(),
            args: HashMap::new(),
            form: HashMap::new(),
            buffer: Vec::new(),
            cache: Rc::new(TypeCache::new()),
        }
    }

    /// Read a raw HTTP request from `reader` and create an
    /// appropriate `Request` to represent it.
    pub fn from_reader(mut reader: TcpStream) -> Result<Request> {
        let mut buffer = Vec::with_capacity(512);
        let mut read_buf = [0u8; 512];

        let mut req = loop {
            let n = reader.read(&mut read_buf)?;
            if n == 0 {
                return Err(error!("Connection Closed"));
            }
            buffer.extend_from_slice(&read_buf[..n]);
            match http_parser::parse(mem::replace(&mut buffer, vec![]))? {
                http_parser::Status::Complete(req) => break req,
                http_parser::Status::Partial(b) => {
                    let _ = mem::replace(&mut buffer, b);
                }
            }
        };

        if let Some(size) = req.header("Content-Length") {
            let size = size.parse().unwrap_or(0);
            let start = req.body.0;
            while req.buffer[start..].len() < size {
                let n = reader.read(&mut read_buf)?;
                if n == 0 {
                    return Err(error!("Connection Closed"));
                }
                req.buffer.extend_from_slice(&read_buf[..n]);
            }
            req.body.1 = req.body.0 + size;
            req.parse_form();
        }

        Ok(req)
    }

    /// Path requested, starting with `/` and not including `?query`.
    pub fn path(&self) -> &str {
        self.path.from_buf(&self.buffer)
    }

    /// Full path requested, starting with `/` and including `?query`.
    pub fn full_path(&self) -> &str {
        self.full_path.from_buf(&self.buffer)
    }

    /// Create a request from an arbitrary path. Used in testing.
    pub fn from_path(path: &str) -> Request {
        Request::default().with_path(path)
    }

    /// Give a request an arbitrary `path`. Can be used in tests or
    /// with `filter`.
    pub fn set_path(&mut self, path: &str) {
        self.full_path = Span(self.buffer.len(), self.buffer.len() + path.len());
        self.buffer.extend(path.as_bytes());
        // path doesn't include ?query
        if let Some(idx) = self.full_path().find('?') {
            self.path = Span(self.full_path.0, self.full_path.0 + idx)
        } else {
            self.path = self.full_path;
        }
    }

    /// Give a request an arbitrary `path`. Can be used in tests or
    /// with `filter`.
    pub fn with_path(mut self, path: &str) -> Request {
        self.set_path(path);
        self
    }

    /// Raw body of HTTP request. If you are using methods like
    /// `with_path` or `set_arg` this will not accurately represent
    /// the raw HTTP request that was made.
    pub fn body(&self) -> &str {
        self.body.from_buf(&self.buffer)
    }

    /// Give this Request an arbitrary body from a string.
    pub fn set_body<S: AsRef<str>>(&mut self, body: S) {
        self.body = Span(self.buffer.len(), self.buffer.len() + body.as_ref().len());
        self.buffer.extend(body.as_ref().as_bytes());
    }

    /// Give this Request an arbitrary body from a string and return
    /// the new Request.
    pub fn with_body<S: AsRef<str>>(mut self, body: S) -> Request {
        self.set_body(body);
        self
    }

    /// HTTP Method
    pub fn method(&self) -> &str {
        self.method.from_buf(&self.buffer)
    }

    /// Give this Request a new HTTP Method.
    pub fn set_method(&mut self, method: &str) {
        self.method = Span(self.buffer.len(), self.buffer.len() + method.len());
        self.buffer.extend(method.as_bytes());
    }

    /// Give this Request a new HTTP Method and return the new Request.
    pub fn with_method(mut self, method: &str) -> Request {
        self.set_method(method);
        self
    }

    /// In a route defined with `routes!` like `"/names/:name"`,
    /// calling `request.arg("name")` will return `Some("peter")` when
    /// the request is `/names/peter`.
    pub fn arg(&self, name: &str) -> Option<&str> {
        self.args.get(name).map(|v| v.as_ref())
    }

    /// Replace or set a new value for an arbitrary URL argument from
    /// a `filter` or in a test.
    pub fn set_arg(&mut self, name: &str, value: &str) {
        self.args.insert(name.to_string(), value.to_string());
    }

    #[doc(hidden)]
    /// For testing. You should use [`header()`](#method.header) to
    /// get a specific header from this Request.
    pub fn headers(&self) -> &Vec<(Span, Span)> {
        &self.headers
    }

    /// Get a header value. `name` is case insensitive.
    pub fn header(&self, name: &str) -> Option<&str> {
        let name = name.to_lowercase();
        self.headers
            .iter()
            .find(|(n, _)| n.from_buf(&self.buffer).to_ascii_lowercase() == name)
            .map(|(_, v)| v.from_buf(&self.buffer).trim())
    }

    /// Was the given form value sent?
    pub fn has_form(&mut self, name: &str) -> bool {
        self.form(name).is_some()
    }

    /// Return a value from the POSTed form data.
    pub fn form(&self, name: &str) -> Option<&str> {
        self.form.get(name).map(|s| s.as_ref())
    }

    /// Replace or set a new value for an arbitrary URL argument from
    /// a `filter` or in a test.
    pub fn set_form(&mut self, name: &str, value: &str) {
        self.form.insert(name.to_string(), value.to_string());
    }

    /// Parse and decode form POST data into a Hash. Should be called
    /// when this Request is created.
    fn parse_form(&mut self) {
        let mut map = HashMap::new();
        for kv in self.body().split('&') {
            let mut parts = kv.splitn(2, '=');
            if let Some(key) = parts.next() {
                if let Some(val) = parts.next() {
                    map.insert(key.to_string(), util::decode_form_value(val));
                } else {
                    map.insert(key.to_string(), String::new());
                };
            }
        }
        self.form = map;
    }

    /// Was the given query value sent?
    pub fn has_query(&mut self, name: &str) -> bool {
        self.query(name).is_some()
    }

    /// Return a value from the ?querystring=
    pub fn query(&self, name: &str) -> Option<&str> {
        let idx = self.full_path().find('?')?;
        self.full_path()[idx + 1..]
            .split('&')
            .filter_map(|s| {
                if s.starts_with(name) && s[name.len()..].starts_with('=') {
                    Some(&s[name.len() + 1..])
                } else {
                    None
                }
            })
            .next()
    }

    /// Request's `cache()` lives for only a single Request, but can
    /// nonethenevertheless be useful to prevent looking up the same
    /// data over and over. The cache is based on the return type of
    /// the function or closure you pass to `cache()` using
    /// [`TypeCache`](struct.TypeCache.html), so make sure to create
    /// little wrapper structs if you want different functions to
    /// return the same common types, like `Vec<String>`:
    ///
    /// ```rust
    /// struct PageNames(Vec<String>);
    /// struct UserNames(Vec<String>);
    /// ```
    ///
    /// Here's an example:
    ///
    /// ```ignore
    /// use vial::prelude::*;
    /// use page::Page;
    /// use db;
    ///
    /// routes! {
    ///     GET "/" => list;
    /// }
    ///
    /// struct PageNames(Vec<String>);
    ///
    /// fn all_pages(_: &Request) -> Vec<Page> {
    ///     db::lookup("select * from pages")
    /// }
    ///
    /// fn page_names(req: &Request) -> PageNames {
    ///     PageNames(req.cache(all_pages)
    ///         .iter()
    ///         .map(|page| page.name.clone())
    ///         .collect::<Vec<_>>())
    /// }
    ///
    /// fn list_of_names(req: &Request) -> String {
    ///     req.cache(page_names)
    ///         .0
    ///         .iter()
    ///         .map(|name| format!("<li>{}</li>", name))
    ///         .collect::<Vec<_>>()
    ///         .join("\n")
    /// }
    ///
    /// fn list(req: Request) -> impl Responder {
    ///     format!(
    ///         "<html>
    ///             <head><title>{title}</title></head>
    ///             <body>
    ///                 <h1>{title}</h1>
    ///                 <h3>There are {page_count} pages:</h3>
    ///                 <ul>
    ///                     {pages}
    ///                 </ul>
    ///             </body>
    ///         </html>",
    ///         title = "List Pages",
    ///         page_count = req.cache(all_pages).len(),
    ///         pages = req.cache(list_of_names),
    ///     )
    /// }
    ///
    /// fn main() {
    ///     run!().unwrap();
    /// }
    /// ```
    pub fn cache<T, F>(&self, fun: F) -> &T
    where
        F: FnOnce(&Request) -> T,
        T: Send + Sync + 'static,
    {
        self.cache.get().unwrap_or_else(|| {
            self.cache.set(fun(&self));
            self.cache.get().unwrap()
        })
    }

    /// Access to global shared state defined with the
    /// [`vial::use_state!`](macro.use_state.html) macro before
    /// starting your application with [`vial::run!`](macro.run.html).
    ///
    /// The `state` feature must also be enabled in `Cargo.toml`.
    ///
    /// ```ignore
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use vial::prelude::*;
    ///
    /// routes! {
    ///     #[filter(count)]
    ///     GET "/" => |req|
    ///         format!("Hits: {}", req.state::<Counter>.hits());
    /// }
    ///
    /// fn count(req: &mut Request) -> Option<Response> {
    ///     req.state::<Counter>().incr();
    ///     None
    /// }
    ///
    /// #[derive(Debug, Default)]
    /// struct Counter(AtomicUsize);
    ///
    /// impl Counter {
    ///     fn hits(&self) -> usize {
    ///         self.0.load(Ordering::Relaxed)
    ///     }
    ///     fn incr(&self) {
    ///         self.0.fetch_add(1, Ordering::Relaxed);
    ///     }
    /// }
    ///
    /// fn main() {
    ///     use_state!(Counter::default());
    ///     run!();
    /// }
    /// ```
    #[cfg(feature = "state")]
    pub fn state<T: Send + Sync + 'static>(&self) -> &T {
        crate::storage::get::<T>()
    }
}
