use crate::util;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        let mut req = Request {
            path: "/".to_string(),
            method: "GET".to_string(),
            params: HashMap::new(),
        };
        req.parse_params();
        req
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    /// Has the given param been set?
    pub fn has_param(&mut self, name: &str) -> bool {
        self.parse_params();
        self.params.contains_key(name)
    }

    /// Return a value in a POST <form> or ?querystring=
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    /// Turn a query string or POST body into a nice and tidy HashMap.
    fn parse_params(&mut self) {
        if !self.params.is_empty() {
            self.params.clear();
        }

        // temp value
        let mut map = HashMap::new();

        // parse url
        let path = self.path();
        if let Some(start) = path.find('?') {
            parse_query_into_map(&path[start + 1..], &mut map);
        }

        // parse POST body
        if self.method() == "POST" {
            todo!();
        }

        if !map.is_empty() {
            self.params = map;
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
