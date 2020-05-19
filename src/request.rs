use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
