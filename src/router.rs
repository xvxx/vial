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
        fn show(r: Request) -> Response {
            format!("Show: {}", r.arg("page").unwrap_or("?")).into()
        }
        fn show_raw(r: Request) -> Response {
            format!("Raw: {}", r.arg("page").unwrap_or("?")).into()
        }
        fn about(_: Request) -> Response {
            "About".into()
        }
        fn info(_: Request) -> Response {
            "Info".into()
        }

        let mut router = Router::new();
        router.insert("GET", "/about", about);
        router.insert("GET", "/:page", show);
        router.insert("GET", "/info", info);
        router.insert("GET", "/:page.md", show_raw);

        let req = Request::from_path("/");
        assert_eq!(router.action_for(&req), None);

        let req = Request::from_path("/cats");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Show: cats".to_string()
        );

        let req = Request::from_path("/dogs");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Show: dogs".to_string()
        );

        let req = Request::from_path("/rabbits?haxcode=1");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Show: rabbits".to_string()
        );

        let req = Request::from_path("/lemurs/?other-haxcode=1&bobby=brown");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Show: lemurs".to_string()
        );

        let req = Request::from_path("/about");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "About".to_string()
        );

        let req = Request::from_path("/info");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Show: info".to_string()
        );

        let req = Request::from_path("/cats.md");
        assert_eq!(
            router.action_for(&req).unwrap()(req).to_string(),
            "Raw: info".to_string()
        );

        let req = Request::from_path("/slashes/dont/match");
        assert_eq!(router.action_for(&req), None);
    }
}
