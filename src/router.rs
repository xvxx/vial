use {
    crate::{Method, Request, Response},
    percent_encoding::percent_decode,
    std::collections::HashMap,
};

/// A Route Pattern is just a list of segements:
///   /my/cool/route => vec!["my", "cool", "route"]
pub type Pattern = Vec<String>;

/// An `Action` is the code we route to. This is slightly different than
/// what's defined using the `routes!` macro - that is more liberal
/// and accepts any `fn(Request) -> impl Responder`, which it then
/// transforms into an `Action` when adding to the `Router`.
pub type Action = fn(Request) -> Response;

/// `Router` keeps track of all the routes defined by
/// [`vial::routes!`](macro.routes.html) and can produce an action for
/// a given HTTP Method and URL path combination using [`action_for`](#method.action_for).
///
/// You never have to create a `Router`, except maybe in testing.
#[derive(Default)]
pub struct Router {
    routes: HashMap<Method, Vec<(Pattern, Action)>>,
}

impl Router {
    /// Create a new `Router`. You shouldn't have to do this.
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    /// Given a [`Request`](struct.Request.html), produce a match as
    /// determined by the calls to
    /// [`vial::routes!`](macro.routes.html) in this application.
    ///
    /// It will also modify the passed `Request` object with any
    /// arguments that may have matched in the URL.
    pub fn action_for(&self, req: &mut Request) -> Option<&Action> {
        if let Some(routes) = self.routes.get(&req.method().into()) {
            let req_parts = Self::pattern_to_vec(req.path());

            'outer: for (pattern, action) in routes {
                for (i, req_part) in req_parts.iter().enumerate() {
                    if i >= pattern.len() {
                        continue 'outer;
                    }
                    if pattern[i].starts_with(':') && !req_part.is_empty() {
                        req.set_arg(
                            pattern[i].trim_start_matches(':'),
                            &percent_decode(req_part.as_bytes()).decode_utf8_lossy(),
                        );
                        continue;
                    } else if pattern[i].starts_with('*') && !req_part.is_empty() {
                        req.set_arg(
                            pattern[i].trim_start_matches('*'),
                            &percent_decode(req_parts[i..].join("/").as_bytes())
                                .decode_utf8_lossy(),
                        );
                        return Some(action);
                    } else if *req_part == pattern[i] {
                        continue;
                    } else {
                        continue 'outer;
                    }
                }
                return Some(action);
            }
        }
        None
    }

    /// Path pattern ("/dogs", "/dogs/:breed") to Vec<String>
    fn pattern_to_vec(pattern: &str) -> Pattern {
        pattern
            .trim_matches('/')
            .split('/')
            .flat_map(|s| s.split('.').map(|s| s.to_string()))
            .collect::<Vec<_>>()
    }

    /// Insert a route into the router. Routes are checked in FIFO
    /// manner when we are trying to match a URL and HTTP Method.
    pub fn insert<T: Into<Method>>(&mut self, method: T, pattern: &'static str, action: Action) {
        let method = method.into();
        let pattern_parts = Self::pattern_to_vec(pattern);

        if let Some(map) = self.routes.get_mut(&method) {
            map.push((pattern_parts, action));
        } else {
            self.routes.insert(method, vec![(pattern_parts, action)]);
        }
    }
}
