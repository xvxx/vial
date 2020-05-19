use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            path: "/".to_string(),
            method: "GET".to_string(),
            params: HashMap::new(),
        }
    }

    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
