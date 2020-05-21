use {
    crate::{asset, Method, Request, Response},
    std::{
        collections::HashMap,
        path::{Path, PathBuf},
    },
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

    pub fn action_for(&self, req: &Request) -> Option<&fn(Request) -> Response> {
        if asset::exists(req.path()) {
            return Some(&Self::serve_static_file);
        }

        if let Some(routes) = self.routes.get(&req.method().into()) {
            if let Some(action) = routes.get(req.path()) {
                return Some(action);
            }
        }
        None
    }

    fn serve_static_file(req: Request) -> Response {
        if let Some(bytes) = asset::read(req.path()) {
            Response::from(bytes)
        } else {
            Response::from(404)
        }
    }

    pub fn insert<T: Into<Method>>(
        &mut self,
        method: T,
        pattern: &str,
        action: fn(Request) -> Response,
    ) {
        let method = method.into();
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
