/// The `vial::run!` macro is the preferred way of starting your Vial
/// application after you've defined one or more routes using
/// [`vial::routes!`](macro.routes.html). `run!` performs a bit of
/// necessary setup, then starts listening for client requests at
/// http://0.0.0.0:7667 by default.
///
/// There are four ways to use `run!`:
///
/// 1. `vial::run!()`: No arguments. Starts listening at
///    http://0.0.0.0:7667 and expects you to have called
///    [`vial::routes!`](macro.routes.html) in the current module.
///
/// 2. `vial::run!("localhost:9999")`: With your own address.
///
/// 3. `vial::run!(blog, wiki)`: With modules that you've called
///    `vial::routes!` from within. This will combine all the routes.
///
///    For example:
///
/// ```no_run
/// mod wiki {
///     vial::routes! {
///         GET "/wiki" => |_| Response::from_file("wiki.html");
///     }
/// }
///
/// mod blog {
///     vial::routes! {
///         GET "/blog" => show_blog;
///         // etc...
///     }
///
///     fn show_blog(req: vial::Request) -> String {
///         // ...
///         "blog".to_string()
///     }
/// }
///
/// fn main() {
///     vial::run!(wiki, blog).unwrap();
/// }
/// ```
///
/// 4. Using a combination of the above:
///
/// ```no_run
/// # mod blog { vial::routes! { GET "/blog" => |_| ""; }}
/// # mod wiki { vial::routes! { GET "/wiki" => |_| ""; }}
///
/// fn main() {
///     vial::run!("localhost:1111", blog, wiki).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! run {
    () => {
        vial::run!("0.0.0.0:7667")
    };
    ($addr:expr) => {{
        vial::run!($addr, self)
    }};
    ($($module:ident),+) => {{
        vial::run!("0.0.0.0:7667", $($module),+)
    }};
    ($addr:expr, $($module:ident),+) => {{
        vial::setup!();
        let mut router = ::vial::Router::new();
        $($module::vial_add_to_router(&mut router);)+
        vial::run($addr, router, None)
    }};
}

/// Same as `vial::run!()`, but allows setting a banner that will be
/// printed to the console when your Vial web app starts.
///
/// You can use {} in place of the server's address. For example:
///
/// ```ignore
/// fn main() {
///     vial::run_with_banner!("--> deadwiki started at {}").unwrap();
/// }
/// ```
///
/// When we start this app, we'll see this:
///
/// ```ignore
/// --> deadwiki started at http://0.0.0.0:7667
/// ```
///
/// Instead of the usual:
///
/// ```ignore
///  ~ vial running at http://0.0.0.0:7667
/// ```
///
#[macro_export]
macro_rules! run_with_banner {
    ($banner:expr) => {
        vial::run_with_banner!($banner, "0.0.0.0:7667")
    };
    ($banner:expr, $addr:expr) => {{
        vial::run_with_banner!($banner, $addr, self)
    }};
    ($banner:expr, $($module:ident),+) => {{
        vial::run_with_banner!($banner, "0.0.0.0:7667", $($module),+)
    }};
    ($banner:expr, $addr:expr, $($module:ident),+) => {{
        vial::setup!();
        let mut router = ::vial::Router::new();
        $($module::vial_add_to_router(&mut router);)+
        vial::run($addr, router, Some($banner))
    }};
}

/// `vial::run_once!`, similarly to `vial::run!`, starts a server.
/// However, this server is started in a *separate thread* and only
/// listens for one request then shuts down. This server is by default
/// bound to an [ephemeral port](https://en.wikipedia.org/wiki/Ephemeral_port).
/// This macro is meant to facilitate easy HTTP
/// [mocking](https://en.wikipedia.org/wiki/Mock_object).
///
/// To be able to know what port is bound to the server instance the
/// macro returns a `String` containing the IP and port combination with
/// "http://" prepended (to allow for easier use) and ending with a '/',
/// for example: `http://127.0.0.1:58231/`.
///
/// The macro also returns a [handle](std::thread::JoinHandle) to the thread that
/// is spawned so it can be properly joined by the user. See [spawn](std::thread::spawn)
/// for an explanation on that.
///
/// Note that even if multiple routes are defined, only one request is processed before the server
/// shuts down.
///
/// `run_once!` accepts the same parameters as run. Below is an example of using `vial::run_once!`.
///
/// ```no_run
/// use minreq;
///
/// mod wiki {
///     vial::routes! {
///         GET "/wiki" => |_| Response::from_file("wiki.html");
///     }
/// }
///
/// mod blog {
///     vial::routes! {
///         GET "/blog" => show_blog;
///         // etc...
///     }
///
///     fn show_blog(req: vial::Request) -> String {
///         // ...
///         "blog".to_string()
///     }
/// }
///
/// fn main() {
///     let (mut addr, thread) = vial::run_once!(wiki, blog);
///     addr.push_str("blog");
///     let resp = minreq::get(addr).send().unwrap();
///     let _ = thread.join();
///     println!("Response: {}", resp.as_str().unwrap());
/// }
/// ```
///
///
#[macro_export]
macro_rules! run_once {
    () => {
        vial::run_once!("127.0.0.1:0")
    };
    ($addr:expr) => {{
        vial::run_once!($addr, self)
    }};
    ($($module:ident),+) => {{
        vial::run_once!("127.0.0.1:0", $($module),+)
    }};
    ($addr:expr, $($module:ident),+) => {{
        vial::setup!();
        let mut router = ::vial::Router::new();
        $($module::vial_add_to_router(&mut router);)+
        vial::run_once($addr, router).unwrap()
    }};
}

/// Gives Vial a state object to manage globally. You can access it
/// by calling
/// [`request.state::<YourStruct>()`](struct.Request.html#method.state)
/// in an action.
///
/// The `vial::use_state!()` macro should be called immediately before
/// calling [`vial::run!()`](macro.run.html) in your application.
///
/// It expects one argument: a `Send + Sync + 'static` object you want
/// to share between all requests.
///
/// ```no_run
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use vial::prelude::*;
///
/// routes! {
///     GET "/" => hello;
///     GET "/count" => count;
/// }
///
/// fn hello(req: Request) -> impl Responder {
///     req.state::<HitCount>().0.fetch_add(1, Ordering::Relaxed);
///     format!("Hits: {}", count(req))
/// }
///
/// fn count(req: Request) -> String {
///     req.state::<HitCount>()
///         .0
///         .load(Ordering::Relaxed)
///         .to_string()
/// }
///
/// #[derive(Default)]
/// struct HitCount(AtomicUsize);
///
/// fn main() {
///     use_state!(HitCount::default());
///     run!().unwrap();
/// }
/// ```
#[macro_export]
macro_rules! use_state {
    ($state:expr) => {
        vial::storage::init();
        vial::storage::set($state);
    };
}

/// This is called by `vial::run!`. You probably should leave it be.
#[doc(hidden)]
#[macro_export]
macro_rules! setup {
    () => {
        #[cfg(bundle_assets)]
        #[macro_export]
        macro_rules! vial_bundled_assets {
            () => { include!(concat!(env!("OUT_DIR"), "/bundle.rs")) };
        }
        #[cfg(bundle_assets)]
        vial::include_assets!();
        vial::asset_dir!(@option option_env!("ASSET_DIR"));
    };
}

/// This is called by `vial::setup!`.
#[doc(hidden)]
#[macro_export]
macro_rules! include_assets {
    () => {
        unsafe {
            vial::BUNDLED_ASSETS = Some(vial_bundled_assets!());
        }
    };
}

/// Vial can serve static files out of an asset directory, complete
/// with ETag support so your browser isn't constantly re-downloading.
///
/// To enable this, put all your `.js` and `.css` and other static
/// assets into a directory in the root of your project, then
/// reference them as if the root of your Vial web application was
/// that asset directory. Next call `vial::asset_dir!()` with the path
/// to your asset directory (maybe `assets/`?) before starting your
/// application with [`vial::run!`](macro.run.html):
///
/// If we had a directory structure like this:
///     .
///     ├── README.md
///     ├── assets
///     │   └── img
///     │       ├── banker.png
///     │       └── doctor.png
///     └── src
///         └── main.rs
///
/// We could serve our images like so:
///
/// ```no_run
/// vial::routes! {
///     GET "/" => |_| "
///         <p><img src='/img/doctor.png'/></p>
///         <p><img src='/img/banker.png'/></p>
///     ";
/// }
///
/// fn main() {
///     vial::asset_dir!("assets/");
///     vial::run!().unwrap();
/// }
/// ```
#[macro_export]
macro_rules! asset_dir {
    (@option $opt:expr) => {
        if let Some(dir) = $opt {
            ::vial::asset_dir!(dir);
        }
    };
    ($dir:expr) => {
        unsafe {
            ::vial::ASSET_DIR = Some($dir.into());
        }
    };
}

/// If you want to bundle your assets into your final binary in
/// release mode, then you need to call `vial::bundle_assets!()` with
/// the path to your asset directory in a `build.rs` file.
///
/// Bundling assets and setting an asset path using
/// [`vial::asset_dir!()`](macro.asset_dir.html) are mutually
/// exclusive - you can't do both, as enabling bundling will set the
/// asset path for you. Therefor if you are making the transition from
/// using-assets-but-not-bundling to using-assets-and-bundling-them,
/// make sure to remove your call to `vial::asset_dir!`.
///
/// To bundle your assets, first add `vial` as a `build-dependency` in
/// your toml file:
///
/// ```toml
/// [build-dependencies]
/// vial = "0.1"
/// ```
///
/// Then either create or open your existing `build.rs` file in the
/// root of your project and call `vial::bundle_assets!` with the path
/// to your asset directory:
///
/// ```no_run
/// fn main() {
///     vial::bundle_assets!("assets/").unwrap();
/// }
/// ```
///
/// This will now bundle your assets in `--release` mode and use the
/// disk in debug and test mode. All calls to functions in the
/// [`assets`](assets/index.html) module should work.
#[macro_export]
macro_rules! bundle_assets {
    ($dir:expr) => {
        ::vial::bundle_assets($dir)
    };
}

/// The `vial::routes!` macro, they say, is half of the battle, with
/// the other 50% being a toss up between "knowledge" and the
/// [`vial::run!`](macro.run.html) macro you use to start your app.
///
/// In Vial, routes are defined within the `routes!` macro in this
/// format:
///
/// > `HTTP_METHOD ROUTE_PATTERN => ACTION;`
///
/// The order in which routes are written matters - routes written
/// first will be checked for matches first, meaning you can declare
/// many routes that point to `"/"`, but only the first one defined
/// will ever match.
///
/// ### HTTP Methods
///
/// `HTTP_METHOD` should be an all caps HTTP method. It will get
/// converted into a [Method](enum.Method.html) enum and can be any
/// one of:
///
/// - `GET`
/// - `HEAD`
/// - `POST`
/// - `PUT`
/// - `DELETE`
/// - `PATCH`
/// - `OPTIONS`
/// - `TRACE`
///
/// ### Route Patterns
///
/// `ROUTE_PATTERN` can be an exact match, such as `"/user"` or
/// `"/v2/search.php3"`, or can include a named URL parameter:
///
/// 1. `"/:name"` — This will match anything except paths with `/` or `.`
///    in them.
/// 2. `"/:name.md"` — Use this format to match on a specific file extension.
/// 3. `"/*name"` — This will match everything, including `/` and `.`
///
/// In the three examples above, calling `request.arg("name")` in an
/// Action will return `Some(&str)`.
///
/// Note that you can have multiple parameters in the same route, as
/// long as the wildcard pattern occurs last:
///
/// ```no_run
/// vial::routes! {
///     GET "/:category/:id/*name" => |req| format!(
///         "<p>Category: {}</p>
///         <p>ID: {}</p>
///         <p>Name: {}</p>",
///         req.arg("category").unwrap_or("None"),
///         req.arg("id").unwrap_or("None"),
///         req.arg("name").unwrap_or("None"),
///     );
/// }
///
/// fn main() {
///     vial::run!();
/// }
/// ```
///
/// ### Actions
///
/// Actions are what routes actually route to.
///
/// They are functions or closures take a
/// [Request](struct.Request.html) and return a
/// [Responder](trait.Responder.html) of some kind.
///
/// ```no_run
/// use vial::prelude::*;
///
/// routes! {
///     GET "/info" => |req| format!(
///         "<p>Name: {}</p>", req.query("name").unwrap_or("None")
///     );
///     GET "/" => index;
/// }
///
/// fn index(req: Request) -> impl Responder {
///     "<form method='GET'>
///         <p>Enter your name: <input type='text' name='name'/></p>
///         <input type='submit'/>
///     </form>"
/// }
///
/// fn main() {
///     run!();
/// }
/// ```
///
/// Returning `impl Responder` is easy -
/// [Responder](trait.Responder.html) is a Vial trait that defines a
/// single conversion method that returns a
/// [Response](struct.Response.html):
///
/// ```rust
/// # use vial::Response;
/// pub trait Responder {
///     fn to_response(self) -> Response;
/// }
/// ```
///
/// These types implement `Responder` by default:
///
/// - `&str`
/// - `String`
/// - `usize` - Empty response with this number as the status code.
/// - `Option<impl Responder>` - 404 on `None`
/// - `Result<impl Responder, Error>` - 500 on Error
///
#[macro_export]
macro_rules! routes {
    (
        $(#![filter($($all_filter:ident),+)])*

        $(
            $(#[filter($($action_filter:ident),+)])*
            $method:ident $path:expr => $body:expr;)*
        ) => {
        fn vial_check_method() {
            #![allow(non_snake_case)]
            fn GET() {}
            fn POST() {}
            fn PUT() {}
            fn DELETE() {}
            fn UPDATE() {}
            fn PATCH() {}
            $($method();)*
        }

        fn vial_filter(req: &mut ::vial::Request) -> Option<::vial::Response> {
            $($({
                if let Some(res) = $all_filter(req) {
                    return Some(res);
                }
            })+)*

            None
        }

        pub fn vial_add_to_router(router: &mut ::vial::Router) {
            $( router.insert(::vial::Method::$method, $path, |mut req| {
                use ::vial::{Request, Response, Responder};

                let b: fn(::vial::Request) -> _ = $body;
                let mut res = vial_filter(&mut req);

                $($({
                    if res.is_none() {
                        res = $action_filter(&mut req);
                    }
                })+)*

                res.unwrap_or_else(|| b(req).to_response())
            }); )*
        }
    };
}
