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

enum ParseStatus {
    Complete(Request),
    Partial(Vec<u8>),
}

#[derive(Debug)]
pub struct Request {
    path: Span,
    full_path: Span,
    method: Span,
    body: Span,
    headers: Vec<(Span, Span)>,
    buffer: Vec<u8>,
    form: HashMap<String, String>,

    pub(crate) args: HashMap<String, String>,
}

impl Request {
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
        }
    }

    pub fn from_reader(mut reader: TcpStream) -> Result<Request> {
        let mut buffer = Vec::with_capacity(512);
        let mut read_buf = [0u8; 512];

        let mut req = loop {
            let n = reader.read(&mut read_buf)?;
            if n == 0 {
                return Err(error!("Connection Closed"));
            }
            buffer.extend_from_slice(&read_buf[..n]);
            match parse(mem::replace(&mut buffer, vec![]))? {
                ParseStatus::Complete(req) => break req,
                ParseStatus::Partial(b) => {
                    mem::replace(&mut buffer, b);
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

    pub fn from_path(path: &str) -> Request {
        Request::default().with_path(path)
    }

    pub fn with_path(mut self, path: &str) -> Request {
        self.path = Span(self.buffer.len(), self.buffer.len() + path.len());
        self.buffer.extend(path.as_bytes());
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
        self.body.from_buf(&self.buffer)
    }

    pub fn method(&self) -> &str {
        self.method.from_buf(&self.buffer)
    }

    pub fn path(&self) -> &str {
        self.path.from_buf(&self.buffer)
    }

    pub fn full_path(&self) -> &str {
        self.full_path.from_buf(&self.buffer)
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
        let name = name.to_lowercase();
        self.headers
            .iter()
            .find(|(n, _)| self.span_as_str(*n).to_ascii_lowercase() == name)
            .and_then(|(_, v)| Some(self.span_as_str(*v).trim()))
    }

    /// Was the given form value sent?
    pub fn has_form(&mut self, name: &str) -> bool {
        self.form(name).is_some()
    }

    /// Return a value from the POSTed form data.
    pub fn form(&self, name: &str) -> Option<&str> {
        self.form.get(name).and_then(|s| Some(s.as_ref()))
    }

    /// Parse and decode form POST data into a Hash.
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
            .split("&")
            .filter_map(|s| {
                if s.starts_with(name) && *&s[name.len()..].chars().next() == Some('=') {
                    Some(&s[name.len() + 1..])
                } else {
                    None
                }
            })
            .next()
    }
}

/// Parse a raw HTTP request into a Request struct.
fn parse(buffer: Vec<u8>) -> Result<ParseStatus> {
    let method_len = loop {
        if buffer.len() < 10 {
            return Ok(ParseStatus::Partial(buffer));
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
        return Ok(ParseStatus::Partial(buffer));
    }
    let path_len = path_len.unwrap();
    let pos = method_len + 1 + path_len + 1;
    if buffer.len() <= pos + 10 {
        return Ok(ParseStatus::Partial(buffer));
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
        return Ok(ParseStatus::Partial(buffer));
    }

    let mut req = Request::default();
    req.method = Span(0, method_len);
    req.full_path = Span(method_len + 1, method_len + 1 + path_len);
    // path doesn't include ?query
    if let Some(idx) = req.full_path.from_buf(&buffer).find('?') {
        req.path = Span(method_len + 1, method_len + 1 + idx)
    } else {
        req.path = req.full_path;
    }
    req.headers = headers;
    req.body = Span(pos, pos + buffer.len());
    req.buffer = buffer;

    Ok(ParseStatus::Complete(req))
}
