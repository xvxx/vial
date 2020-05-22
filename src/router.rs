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

fn serve_static_file(req: Request) -> Response {
    Response::from_file(req.path())
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn action_for(&self, req: &Request) -> Option<&fn(Request) -> Response> {
        if asset::exists(req.path()) {
            return Some(&(serve_static_file as fn(Request) -> Response));
        }

        if let Some(routes) = self.routes.get(&req.method().into()) {
            if let Some(action) = routes.get(req.path()) {
                return Some(action);
            }
        }
        None
    }

    pub fn insert<T: Into<Method>>(
        &mut self,
        method: T,
        pattern: &str,
        action: fn(Request) -> Response,
    ) {
        let method = method.into();
        if let Some(map) = self.routes.get_mut(&method) {
            // don't overwrite routes. ie first "/" defined wins
            if !map.contains_key(pattern) {
                map.insert(pattern.to_string(), action);
            }
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
