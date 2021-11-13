use {
    crate::{util::percent_decode, Method, Request, Response},
    std::cmp::max,
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
    #[must_use]
    pub fn new() -> Self {
        Self {
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
                for i in 0..max(req_parts.len(), pattern.len()) {
                    if i >= pattern.len() {
                        continue 'outer;
                    }
                    let pattern_part = &pattern[i];
                    if i >= req_parts.len() {
                        continue 'outer;
                    }
                    let req_part = &req_parts[i];
                    if pattern_part.starts_with(':') && !req_part.is_empty() {
                        if let Some(decoded) = percent_decode(req_part) {
                            req.set_arg(pattern_part.trim_start_matches(':').into(), decoded);
                        }
                        continue;
                    } else if pattern_part.starts_with('*') && !req_part.is_empty() {
                        if let Some(idx) = req.path().find(&req_parts[i]) {
                            if let Some(decoded) = percent_decode(&req.path()[idx..]) {
                                req.set_arg(pattern_part.trim_start_matches('*').into(), decoded);
                            }
                        }
                        return Some(action);
                    } else if req_part == pattern_part {
                        continue;
                    }
                    continue 'outer;
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
            .flat_map(|s| {
                s.find('.').map_or_else(
                    || vec![s.to_string()],
                    |idx| vec![s[..idx].to_string(), s[idx..].to_string()],
                )
            })
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
