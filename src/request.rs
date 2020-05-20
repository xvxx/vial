use crate::util;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub body: String,

    pub query: HashMap<String, String>,
    pub form: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            path: "/".to_string(),
            method: "GET".to_string(),
            body: "".to_string(),
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
