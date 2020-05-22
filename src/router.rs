use {
    crate::{asset, Method, Request, Response},
    std::{
        collections::HashMap,
        path::{Path, PathBuf},
    },
};

#[derive(Default)]
pub struct Router {
    routes: HashMap<Method, Vec<(Vec<String>, fn(Request) -> Response)>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn action_for(&self, req: &mut Request) -> Option<&fn(Request) -> Response> {
        if let Some(routes) = self.routes.get(&req.method().into()) {
            let req_parts = Self::pattern_to_vec(req.path());

            'outer: for (pattern, action) in routes {
                req.args.clear();
                if pattern.len() == req_parts.len() {
                    for (i, req_part) in req_parts.iter().enumerate() {
                        if pattern[i].starts_with(':') && !req_part.is_empty() {
                            req.args.insert(
                                pattern[i].trim_start_matches(':').to_string(),
                                percent_encoding::percent_decode(req_part.as_bytes())
                                    .decode_utf8_lossy()
                                    .to_string(),
                            );
                            continue;
                        } else if *req_part == pattern[i] {
                            continue;
                        } else {
                            continue 'outer;
                        }
                    }
                    return Some(action);
                }
            }
        }
        None
    }

    /// Path pattern ("/dogs", "/dogs/:breed") to Vec<String>
    fn pattern_to_vec(pattern: &str) -> Vec<String> {
        pattern
            .trim_matches('/')
            .split('/')
            .flat_map(|s| s.split('.').map(|s| s.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn insert<T: Into<Method>>(
        &mut self,
        method: T,
        pattern: &'static str,
        action: fn(Request) -> Response,
    ) {
        let method = method.into();
        let pattern_parts = Self::pattern_to_vec(pattern);

        if let Some(map) = self.routes.get_mut(&method) {
            map.push((pattern_parts, action));
        } else {
            self.routes.insert(method, vec![(pattern_parts, action)]);
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

        let mut req = Request::from_path("/");
        assert_eq!(router.action_for(&mut req), None);

        let mut req = Request::from_path("/cats");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Show: cats".to_string()
        );

        let mut req = Request::from_path("/dogs");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Show: dogs".to_string()
        );

        let mut req = Request::from_path("/rabbits?haxcode=1");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Show: rabbits".to_string()
        );

        let mut req = Request::from_path("/lemurs/?other-haxcode=1&bobby=brown");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Show: lemurs".to_string()
        );

        let mut req = Request::from_path("/about");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "About".to_string()
        );

        let mut req = Request::from_path("/info");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Show: info".to_string()
        );

        let mut req = Request::from_path("/cats.md");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Raw: cats".to_string()
        );

        let mut req = Request::from_path("/cats and dogs.md");
        assert_eq!(
            router.action_for(&mut req).unwrap()(req).to_string(),
            "Raw: cats and dogs".to_string()
        );

        let mut req = Request::from_path("/slashes/dont/match");
        assert_eq!(router.action_for(&mut req), None);
    }
}
