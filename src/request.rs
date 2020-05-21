use crate::{util, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    path: String,
    method: String,
    body: String,

    headers: HashMap<String, String>,
    args: HashMap<String, String>,
    query: HashMap<String, String>,
    form: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            path: "/".to_string(),
            method: "GET".to_string(),
            body: "".to_string(),
            headers: HashMap::new(),
            args: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
        }
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn arg(&self, name: &str) -> Option<&str> {
        self.args.get(name).and_then(|v| Some(v.as_ref()))
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| Some(v.as_ref()))
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
        let mut map = HashMap::new();
        parse_query_into_map(&self.body, &mut map);
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
            if let Some(idx) = self.path.find('?') {
                self.path = self.path[..idx].to_string();
            }
            self.query = map;
        }
    }

    pub(crate) fn from_raw_http_request(buf: &[u8]) -> Result<Option<Request>> {
        let mut headers = [httparse::EMPTY_HEADER; 100];
        let mut hreq = httparse::Request::new(&mut headers);

        let headers = hreq
            .parse(buf)
            .map_err(|e| error!("Unable to parse HTTP headers: {}", e))?;

        let header_length = match headers {
            httparse::Status::Complete(n) => n,
            httparse::Status::Partial => return Ok(None),
        };

        let mut req = Request::new();
        for header in hreq.headers {
            req.headers.insert(
                header.name.to_string(),
                String::from_utf8_lossy(header.value).to_string(),
            );
        }
        if let Some(method) = hreq.method {
            req.method = method.into();
        }
        if let Some(path) = hreq.path {
            req.path = path.into();
            req.parse_query();
        }
        if header_length < buf.len() {
            req.body = String::from_utf8_lossy(&buf[header_length..]).into();
            req.parse_body();
        }

        Ok(Some(req))
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
