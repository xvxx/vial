use {
    crate::{Method, Request, Response},
    std::collections::HashMap,
};

#[derive(Default)]
pub struct Router {
    routes: HashMap<Method, HashMap<String, fn(Request) -> Response>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, method: &str, pattern: &str, action: fn(Request) -> Response) {
        let method = Method::from(method);
        if let Some(map) = self.routes.get_mut(&method) {
            map.insert(pattern.to_string(), action);
        } else {
            let mut map = HashMap::new();
            map.insert(pattern.to_string(), action);
            self.routes.insert(method, map);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Request, Response};

    #[test]
    fn test() {
        fn show(_: Request) -> Response {
            "Show".into()
        }
        fn show_raw(_: Request) -> Response {
            "Raw".into()
        }

        let mut router = Router::new();
        router.insert("GET", "/:page", show);
        router.insert("GET", "/:page", show_raw);
    }
}
