use crate::{util, Result};
use std::{
    collections::HashMap,
    io::{self, Read},
    mem,
    net::TcpStream,
    str,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Span(usize, usize);

impl Span {
    pub fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub fn from_buf<'buf>(&self, buf: &'buf [u8]) -> &'buf str {
        if self.1 >= self.0 && self.1 <= buf.len() {
            str::from_utf8(&buf[self.0..self.1]).unwrap_or("?")
        } else {
            ""
        }
    }
}

#[derive(Debug)]
pub struct Request {
    path: Span,
    method: Span,
    body: Span,
    headers: Vec<(Span, Span)>,
    buffer: Vec<u8>,

    pub(crate) args: HashMap<String, String>,
    query: HashMap<String, String>,
    form: HashMap<String, String>,
}

enum Parse {
    Complete(Request),
    Partial(Vec<u8>),
}

impl Request {
    pub fn default() -> Request {
        Request {
            path: Span(0, 0),
            method: Span(0, 0),
            body: Span(0, 0),
            headers: Vec::new(),
            args: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
            buffer: Vec::new(),
        }
    }

    pub fn from_reader(mut reader: TcpStream) -> Result<Request> {
        let mut buffer = Vec::with_capacity(512);
        let mut read_buf = [0u8; 512];

        loop {
            let n = reader.read(&mut read_buf)?;
            if n == 0 {
                return Err(error!("Connection Closed"));
            }
            buffer.extend_from_slice(&read_buf[..n]);
            match parse(mem::replace(&mut buffer, vec![]))? {
                Parse::Complete(req) => return Ok(req),
                Parse::Partial(b) => {
                    mem::replace(&mut buffer, b);
                }
            }
        }
    }

    pub fn from_path(path: &str) -> Request {
        Request::default().with_path(path)
    }

    pub fn with_path(mut self, path: &str) -> Request {
        self.path = Span(self.buffer.len(), self.buffer.len() + path.len());
        self.buffer.extend(path.as_bytes());
        self.parse_query();
        self
    }

    pub fn with_body(mut self, body: &str) -> Request {
        self.body = Span(self.buffer.len(), self.buffer.len() + body.len());
        self.buffer.extend(body.as_bytes());
        self
    }

    pub fn with_method(mut self, method: &str) -> Request {
        self.method = Span(self.buffer.len(), self.buffer.len() + method.len());
        self.buffer.extend(method.as_bytes());
        self
    }

    pub fn body(&self) -> &str {
        str::from_utf8(&self.buffer[self.body.0..self.body.1]).unwrap_or("?")
    }

    pub fn method(&self) -> &str {
        str::from_utf8(&self.buffer[self.method.0..self.method.1]).unwrap_or("?")
    }

    pub fn path(&self) -> &str {
        str::from_utf8(&self.buffer[self.path.0..self.path.1]).unwrap_or("?")
    }

    pub fn arg(&self, name: &str) -> Option<&str> {
        self.args.get(name).and_then(|v| Some(v.as_ref()))
    }

    fn span_as_str(&self, span: Span) -> &str {
        if span.1 < self.buffer.len() && span.1 >= span.0 {
            str::from_utf8(&self.buffer[span.0..span.1]).unwrap_or("?")
        } else {
            ""
        }
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        println!("header: {}", name);
        println!(
            "headers: {:?}",
            self.headers
                .iter()
                .map(|(n, v)| format!("{} = {};", self.span_as_str(*n), self.span_as_str(*v)))
                .collect::<Vec<_>>()
                .join(" ")
        );
        self.headers
            .iter()
            .find(|(n, _)| self.span_as_str(*n).to_lowercase() == name)
            .and_then(|(_, v)| Some(self.span_as_str(*v).trim()))
    }

    /// Was the given form value sent?
    pub fn has_form(&mut self, name: &str) -> bool {
        self.form.contains_key(name)
    }

    /// Return a value from the POSTed form data.
    pub fn form(&self, name: &str) -> Option<&str> {
        self.form.get(name).and_then(|s| Some(s.as_ref()))
    }

    /// Turn POSTed form data into a nice 'n tidy HashMap.
    pub(crate) fn parse_body(&mut self) {
        if !self.form.is_empty() {
            self.form.clear();
        }
        if self.body.is_empty() {
            return;
        }
        let mut map = HashMap::new();
        parse_query_into_map(self.body(), &mut map);
        if !map.is_empty() {
            self.form = map;
        }
    }

    /// Was the given query value sent?
    pub fn has_query(&mut self, name: &str) -> bool {
        self.query.contains_key(name)
    }

    /// Return a value from the ?querystring=
    pub fn query(&self, name: &str) -> Option<&str> {
        self.query.get(name).and_then(|s| Some(s.as_ref()))
    }

    /// Turn a query string into a nice 'n tidy HashMap.
    pub(crate) fn parse_query(&mut self) {
        if !self.query.is_empty() {
            self.query.clear();
        }

        // temp value
        let mut map = HashMap::new();

        // parse url
        let path = self.path();
        if let Some(start) = path.find('?') {
            parse_query_into_map(&path[start + 1..], &mut map);
        }

        if !map.is_empty() {
            // strip ?querystring from /path
            if let Some(idx) = self.path().find('?') {
                self.path = Span(self.path.0, self.path.0 + idx);
            }
            self.query = map;
        }
    }
}

/// Parses a query string like "name=jimbo&other_data=sure" into a
/// map!("name" => "jimbo", "other_data" => "sure") HashMap
fn parse_query_into_map(params: &str, map: &mut HashMap<String, String>) {
    for kv in params.split('&') {
        let mut parts = kv.splitn(2, '=');
        if let Some(key) = parts.next() {
            if let Some(val) = parts.next() {
                map.insert(key.to_string(), util::decode_form_value(val));
            } else {
                map.insert(key.to_string(), "".to_string());
            }
        }
    }
}

/// Parse a raw HTTP request into a Request struct.
fn parse(buffer: Vec<u8>) -> Result<Parse> {
    let method_len = loop {
        if buffer.len() < 10 {
            return Ok(Parse::Partial(buffer));
        }
        match &buffer[0..3] {
            b"GET" | b"PUT" => break 3,
            b"HEA" | b"POS" => match &buffer[0..4] {
                b"HEAD" | b"POST" => break 4,
                _ => {}
            },
            b"PAT" | b"TRA" => match &buffer[0..5] {
                b"PATCH" | b"TRACE" => break 5,
                _ => {}
            },
            b"DEL" => {
                if &buffer[0..6] == b"DELETE" {
                    break 6;
                }
            }
            b"CON" | b"OPT" => match &buffer[0..7] {
                b"CONNECT" | b"OPTIONS" => break 7,
                _ => {}
            },

            _ => {}
        }

        return Err(error!("Unknown HTTP method"));
    };

    let path_len = buffer[method_len + 1..].iter().position(|c| *c == b' ');
    if path_len.is_none() {
        return Ok(Parse::Partial(buffer));
    }
    let path_len = path_len.unwrap();

    let pos = method_len + 1 + path_len + 1;

    if buffer.len() <= pos + 10 {
        return Ok(Parse::Partial(buffer));
    }
    if &buffer[pos..pos + 8] != b"HTTP/1.1" {
        return Err(error!(
            "Error parsing HTTP: {}",
            str::from_utf8(&buffer).unwrap_or("???")
        ));
    }
    let pos = pos + 8;
    if &buffer[pos..pos + 2] != b"\r\n" {
        return Err(error!("Error parsing HTTP: expected \\r\\n"));
    }

    let mut pos = pos + 2;
    let mut start = pos;
    let mut headers = Vec::with_capacity(16);
    let mut name = Span(0, 0);
    let mut saw_end = false;
    let mut parsing_key = true;

    let mut iter = buffer[pos..].iter();
    while let Some(c) = iter.next() {
        if parsing_key {
            match *c {
                b':' => {
                    name = Span(start, pos);
                    start = pos + 1;
                    parsing_key = false;
                }
                b'\r' | b'\n' | b' ' => return Err(error!("Error parsing HTTP: header key")),
                _ => {}
            }
        } else {
            match *c {
                b'\r' => {
                    if buffer.get(pos + 1) == Some(&b'\n') {
                        if name == Span(0, 0) {
                            return Err(error!("Error parsing HTTP"));
                        }

                        headers.push((name, Span(start, pos)));
                        name = Span(0, 0);
                        iter.next();
                        parsing_key = true;

                        if buffer.get(pos + 2) == Some(&b'\r')
                            && buffer.get(pos + 3) == Some(&b'\n')
                        {
                            pos += 4;
                            saw_end = true;
                            break;
                        }

                        start = pos + 2;
                        pos += 1;
                    }
                }
                _ => {}
            }
        }
        pos += 1;
    }

    // didn't receive full headers, abort
    if !saw_end {
        return Ok(Parse::Partial(buffer));
    }

    let mut req = Request::default();
    req.method = Span(0, method_len);
    req.path = Span(method_len + 1, method_len + 1 + path_len);
    req.headers = headers;
    req.buffer = buffer;
    if let Some(size) = req.header("Content-Length") {
        req.body = Span(pos, size.parse().unwrap_or(0));
    }

    Ok(Parse::Complete(req))
}
