<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# Vial

#### ~ a micro micro-framework ~

**Vial** is a small web framework for making small web "sites" in
Rust. It includes just a handful of basic features for delivering old
school, server-side rendered HTML: request routing, form data parsing,
response building, and serving static file assets.

The goal is a small, lean core that compiles quickly and has as few
dependencies as possible. Use it for HTML stuff: prototyping ideas,
testing out concepts, or, perhaps, even writing tiny personal apps.
Nothing serious though, got it?

This manual is an overview of **Vial**’s built-in features, as well as
the few _optional_ features you can enable. It also includes
suggestions for some "common tasks", like using a database to store
information.

## Hello World

Here's the bare minimum:

```rust
vial::routes! {
    GET "/" => |_| "Greetings, creature.";
}

fn main() {
    vial::run!();
}
```

That should tell you a lot, in that there isn't a lot to **Vial**.

Now here's a bigger bear, showing off more of **Vial**'s features:

```rust
use vial::prelude::*;

routes! {
    GET "/" => hello_world;
    POST "/" => redirect_to_greeting;
    GET "/:name" => hello_name;
    GET "/*path" => |req|
      Response::from(404).with_body(
        format!("<h1>404 Not Found: {}</h1>",
          req.arg("path").unwrap_or("")));
}

fn hello_world(_req: Request) -> &'static str {
    "<h1>Hello, world!</h1>
    <p><strong>What's your name?</strong></p>
    <form method='POST' action='/'>
        <p><input name='name' type='text'/></p>
        <p><input type='submit'/></p>
    </form>"
}

fn redirect_to_greeting(req: Request) -> Option<impl Responder> {
    let name = req.form("name")?;
    Some(Response::redirect_to(format!("/{}", name)))
}

fn hello_name(req: Request) -> String {
    format!(
        "<h1>Why hello there, {}!</h1>",
        req.arg("name").unwrap()
    )
}

fn main() {
    run!().unwrap();
}
```

You can run the above example from the root of this repository:

    $ cargo run --example manual

**Vial** comes with a handful of examples in the `examples/`
directory, so be sure to peruse them skeptically - either alongside
or after digesting this manual.

## Overview

Like most web library thingy-jingies that only focus on server-side
rendering, there are three main parts to a **Vial** application:

- **[Routing]**: You write actions that take a [Request] and return
  either a [Response] or a [Responder], then map them to URLs and
  URL patterns using the [vial::routes!][routes api] macro.

- **[Requests]**: The [Request] object provides information about each
  client's humble request.

- **[Responses]**: Your actions return either a [Response] struct,
  which can be easily built, or a type that implements the [Responder]
  trait, like `String` or `Option<Response>`.

## Getting Started

**Vial** should work on any recent, stable version of **Rust** on
**Linux** or **macOS**.

To begin, add **Vial** to your project's `Cargo.toml`:

```rust
[dependencies]
vial = "0.1.0"
```

Now all you have to do is call `vial::routes!` to define your routes
and `vial::run!` to start the server in `src/main.rs`:

```rust
vial::routes! {
    GET "/" => |_| "It works!";
}

fn main() {
    vial::run!();
}
```

This should start a server at <http://0.0.0.0:7667> and tell you that
it did. Congratulations! You're on your way.

## Routing

Routing is the real gravy and potatoes of any web framework, if you
think about it. In **Vial**, routes are defined with the
[vial::routes!][routes api] macro in this format:

    HTTP_METHOD ROUTE_PATTERN => ACTION;

The order in which routes are written matters - routes written first
will be checked for matches first, meaning you can declare many routes
that point to `"/"`, but only the first one defined will ever match.

### HTTP Methods

`HTTP_METHOD` can be one of:

- `GET`
- `HEAD`
- `POST`
- `PUT`
- `DELETE`
- `PATCH`

### Route Patterns

`ROUTE_PATTERN` can be an exact match, such as `"/user"` or
`"/v2/search.php3"`, or it can include a named parameter:

1. `"/:name"` — This will match anything except paths with `/` or `.`
   in them.
2. `"/:name.md"` — Use this format to match on a specific file extension.
3. `"/*name"` — This will match everything, including `/` and `.`

In the three examples above, calling `request.arg("name")` in an
Action will return `Some(&str)`.

Note that you can have multiple parameters in the same route, as long
as the "match all" pattern occurs last:

```rust
vial::routes! {
    GET "/:category/:id/*name" => |req| format!(
        "<p>Category: {}</p>
        <p>ID: {}</p>
        <p>Name: {}</p>",
        req.arg("category").unwrap_or("None"),
        req.arg("id").unwrap_or("None"),
        req.arg("name").unwrap_or("None"),
    );
}

fn main() {
    vial::run!();
}
```

### Actions

Actions are what routes actually route to. They are functions or
closures take a [Request] and return either a [Response] or something
that implements the [Responder] trait:

```rust
use vial::prelude::*;

routes! {
    GET "/info" => |req| format!(
        "<p>Name: {}</p>", req.query("name").unwrap_or("None")
    );
    GET "/" => index;
}

fn index(req: Request) -> &'static str {
    "<form method='GET'>
        <p>Enter your name: <input type='text' name='name'/></p>
        <input type='submit'/>
    </form>"
}

fn main() {
    run!();
}
```

Returning `impl Responder` is easy - [Responder] is a **Vial** trait
that defines a single conversion method that returns a [Response]:

```rust
pub trait Responder {
    fn to_response(self) -> Response;
}
```

These types implement `Responder` by default:

- `&str`
- `String`
- `usize` - Empty response with this number as the status code.
- `Option<impl Responder>` - 404 on `None`
- `Result<impl Responder, Error>` - 500 on Error

### Filters

Filters are functions that are run before actions. They can either
modify the existing request before it's sent to your action, or they
can return a `Response` that will be delivered to the client without any
actions being called:

```rust
fn(req: &mut Request) -> Option<Response>;
```

Like Rust's attributes, they can either apply to all routes defined in
the same `vial::routes!` macro call or just a specific route:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use vial::prelude::*;

routes! {
    // `count` will run before all routes in this block
    #![filter(count)]

    GET "/" => |_| "Hey there!";
    GET "/hits" => hits;

    // `count` will run again when /double is visited
    #[filter(count)]
    GET "/double" => double;

    // `echo` will be called when /echo is visited
    #[filter(echo)]
    GET "/echo" => |_| "Is there an echo in here?";
}

fn hits(req: Request) -> impl Responder {
    format!("Hits: {}", req.counter().count())
}

fn double(req: Request) -> impl Responder {
    "Double trouble."
}

fn echo(req: &mut Request) -> Option<Response> {
    println!("{:#?}", req);
    None
}

fn count(req: &mut Request) -> Option<Response> {
    req.counter().incr();
    None
}

#[derive(Debug, Default)]
struct Counter(AtomicUsize);

impl Counter {
    fn count(&self) -> String {
        self.0.load(Ordering::Relaxed).to_string()
    }

    fn incr(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

trait WithCounter {
    fn counter(&self) -> &Counter;
}

impl WithCounter for Request {
    fn counter(&self) -> &Counter {
        self.state::<Counter>()
    }
}

fn main() {
    use_state!(Counter::default());
    run!().unwrap();
}
```

### Route Modules

Routes can be defined in different modules and combined together with
`vial::run!`:

```rust
mod blog;

mod wiki {
    vial::routes! {
        GET "/wiki" => |_| "This is the wiki.";
    }
}

vial::routes! {
    GET "/" => |_| "Index page.";
}

fn main() {
    vial::run!(self, blog, wiki);
}
```

## Requests

When a route matches and an Action is called, it's passed a
[Request] object. `Request` contains information about
the request itself, as well as a number of helper methods.

### Route Parameters

As mentioned in the [Routing] section above, you can define parameters
in a route and access their value for a given request using
`request.arg()`:

```rust
vial::routes! {
    GET "/:animal" => |req| format!(
        "Animal: {}", req.arg("animal").unwrap_or("None")
    );
}
```

### Query Parameters

In addition to route parameters, **Vial** will also parse good ol'
fashioned query string parameters for you:

```rust
vial::routes! {
    GET "/info" => |req| format!(
        "Version: v{}",
        req.query("version").unwrap_or("?")
    );
}

fn main() {
    vial::run!();
}
```

Running this and visiting `/info` will show:

    Version: v?

But visiting `/info?version=1.0` will show:

    Version: v1.0

Like `arg()`, `query()` returns `Option<&str>`.

### Form Data

What's the web without open ended `<textarea>s`? Perish the thought.

POSTed form data follows the same pattern as query and route
parameters: use `request.form()` to access a form parameter:

```rust
use vial::prelude::*;
use db;

routes! {
    GET "/show/:id" => show;
    GET "/new" => new;
    POST "/new" => create;
}

fn new(_req: Request) -> impl Responder {
    "<form method='POST'>
        <p>Name: <input type='text' name='name'/></p>
        <p>Location: <input type='text' name='location'/></p>
        <p><input type='submit'/></p>
    </form>"
}

fn create(req: Request) -> Result<impl Responder, io::Error> {
    let id = db::insert!(
        "name" => req.form("name").unwrap(),
        "location" => req.form("location").unwrap()
    )?;
    Ok(Response::redirect_to(format!("/show/{}", id)))
}

fn show(req: Request) -> Option<impl Responder> {
    let record = db::query!("id" => id).ok()?;
    format!(
        "<p>Name: {}</p>
        <p>Location: {}</p>",
        record.get("name").unwrap_or("None"),
        record.get("location").unwrap_or("None"),
    )
}

fn main() {
    run!();
}
```

### Request Headers

Headers are available without any of the peksy conveniences of type
safety. Just give `request.header()` a string and hope you get one
back!

```rust
use vial::prelude::*;
use std::{fs, path::Path};

routes! {
    GET "/:file" => show;
}

fn show(req: Request) -> Option<impl Responder> {
    let path = format!("./{}", req.arg("file")?);
    if Path::new(&path).exists() {
        if req.header("Accept").unwrap_or("?").starts_with("text/plain") {
            Some(Response::from_header("Content-Type", "text/plain")
                .with_file(&path))
        } else {
            let file = fs::read_to_string(&path).unwrap();
            Some(Response::from_body(format!(
                "<html><body>
                <pre style='width:50%;margin:0 auto'>{}</pre>
                </body></html>", file)))
        }
    } else {
        None
    }
}

fn main() {
    run!();
}
```

Header names are case insensitive, though, so at least you don't have
to worry about that.

### Other Info

Beyond the headers, `Request` also surfaces a few more basic bits of
information such as the `request.method()` and `request.path()`:

```rust
impl Request {
    // "GET", "POST", etc. Always uppercase.
    fn method(&self) -> &str;
    // Always starts with "/"
    fn path(&self) -> &str;
}
```

## Responses

Every Action returns either a [Response] or a type that implements the
[Responder] trait's single method:

```rust
pub trait Responder {
    fn to_response(self) -> Response;
}
```

Common types like `&str` and `Option<String>` already implement this,
so you are free to be lazy and return simple types in your Actions.
If, however, you want to set headers and do other fancy jazz, you'll
need to build and return a [Response] directly.

Rather than use the "Builder" pattern like more mature and better
designed libraries, **Vial's** [Response] lets you set properties
either directly or using Builder-style methods:

```rust
vial::routes! {
    GET "/404" => |_| Response::from(404).with_text("404 Not Found");
}
```

Each `Response` defaults to a `Content-Type` of `text/html; charset=utf8`, so you can build HTML with your bare hands:

```rust
fn index(_req: Request) -> impl Responder {
    Response::from("<marquee>Coming soon!</marquee>")
}
```

To produce plain text, set the header using `with_header()` or
`set_header()`, or use `with_text()` instead of `with_body()` or
`from_body()`:

```rust
fn readme(_req: Request) -> Response {
    // This will be rendered as plain text.
    Response::from_file("README.md")
        .with_header("Content-Type", "text/plain")
}
```

### Building Responses

The [Response][response api] documentation contains more information
on all the methods available, but here are some of the properties you
can set on a [Response] in your actions:

- `fn from_text<S: AsRef<str>>(text: S) -> Response;`

- `fn with_code(mut self, code: usize) -> Response;`

- `fn with_body<S: AsRef<str>>(mut self, body: S) -> Response;`

- `fn with_text<S: AsRef<str>>(self, text: S) -> Response;`

- `fn with_reader(mut self, reader: Box<dyn io::Read>) -> Response;`

- `fn with_asset(mut self, path: &str) -> Response;`

- `fn with_file(mut self, path: &str) -> Response;`

- `fn with_error<E: error::Error>(self, err: E) -> Response;`

- `fn with_header(mut self, key: &str, value: &str) -> Response;`

### Redirect

To issue a 302 redirect, use the `redirect_to` static method:

```rust
fn search(req: Request) -> Option<impl Responder> {
    let url = format!(
        "https://en.wikipedia.org/wiki/Special:Search?search={}",
        req.arg("search")?
    )
    Some(Response::redirect_to(url))
}
```

### Status Codes

Empty responses with status codes can be created from `usize`:

```rust
fn fourohno(_req: Request) -> impl Responder {
    Response::from(404)
}

fn pay_me(_req: Request) -> impl Responder {
    402
}
```

### Headers

Headers can be set Builder-style using `with_header` or
imperative-style using `set_header`:

```rust
fn not_found(_req: Request) -> Response {
    Response::from(404)
        .with_header("Content-Type", "text/plain")
        .with_body("404 Not Found")
}

fn download(req: Request) -> Option<impl Responder> {
    Response::from_file(req.arg("file")?)
        .with_header("Content-Type", "application/octet-stream")
}

fn perm_redirect(url: &str) -> Response {
    Response::from(301).with_header("Location", url)
}
```

## Serving Static Files

**Vial** can automatically serve static files out of an asset
directory, complete with proper ETag handling, if you tell it which
directory to use with the `vial::asset_dir!` macro. It can also
optionally bundle those assets them into your application in
`--release` mode, producing a single binary that was developed as if
it used separate CSS and JS files.

### `vial::asset_dir!`

To get started, put all your `.js` and `.css` and other static
assets into a directory in the root of your project, such as `assets/`.
In your HTML, include those files as if the "assets" directory were
the root of your application. So if you have "assets/app.js", in your
`<script>` tag you would include just "/app.js".

Next call [vial::asset_dir!()][asset_dir api] with the
path to your asset directory (maybe `assets/`?) before starting
your application with [vial::run!](run api):

If we had a directory structure like this:

    .
    ├── README.md
    ├── assets
    │   └── img
    │       ├── banker.png
    │       └── doctor.png
    └── src
        └── main.rs

We could serve our images like so:

```rust
vial::routes! {
    GET "/" => |_| "
        <p><img src='/img/doctor.png'/></p>
        <p><img src='/img/banker.png'/></p>
    ";
}
fn main() {
    vial::asset_dir!("assets/");
    vial::run!().unwrap();
}
```

### `asset::methods()`

By setting an asset directory, either through the
[vial::asset_dir!()][asset_dir api] or
[vial::bundle_assets!()][bundle_assets api] macro, you can then use
the methods in the `asset::` module to work with them:

- **[asset::etag()][etag api]**: Get the ETag for an asset.
  Used automatically by the Router if a web request matches an
  asset's path.

- **[asset::exists()][exists api]**: Does an asset exist?
  Works regardless of whether the asset is bundled or not.

- **[asset::is_bundled()][is_bundled api]**: Are assets
  bundled? Only true in `--release` mode and when used with the
  `vial::bundle_assets!()` macro.

- **[asset::to_string()][to_string api]**: Like
  `fs::read_to_string()`, delivers the content of an asset as a
  `String`.

- **[asset::as_reader()][as_reader api]**: Like
  `asset::to_string()` but provides an `io::Read` of an asset,
  whether or not it's bundled.

### Bundling Assets

**Vial** is meant to be small and swift, like a ninja star. Part of
that means **Vial** apps should be able to compile into _standalone
binaries_ that don't rely on the filesystem. In order to accomplish
this, **Vial** can bundle your assets into your final `--release`
binary for you. As long as you the `asset::()` API described above to
access them, all your code will work the same whether your assets are
bundled or not.

Combined with the ETag support, this means all assets will be reloaded
by your browser whenever they're modified in dev mode for a smoother
developmental experience.

To bundle your assets into your final binary in release mode, you
must:

1. **REMOVE** any calls to the `vial::asset_dir!()` macro.

2. Add `vial` as to `[build-dependencies]` in your `Cargo.toml`:

```toml
[build-dependencies]
vial = "0.1.0"
```

(_Yes, you should now have `vial` in there twice. Once for
`build-dependencies` and once for `dependencies`. Only the one under
`dependencies` needs to list any optional features you want to use,
however._)

3. Create a `bundle.rs` and call `vial::bundle_assets!()` in it,
   passing your asset directory as the sole argument:

```rust
fn main() {
    vial::bundle_assets!("assets/").unwrap();
}
```

**⚠️ Note:** Bundling assets and setting an asset path using
`vial::asset_dir!()` are mutually exclusive - you can't do both, as
enabling bundling will set the asset path for you. Therefor if you
are making the transition from using-assets-but-not-bundling to
using-assets-and-bundling-them, make sure to remove your call to `vial::asset_dir!`.

Other than that, you're all set! Your application will now bundle your
assets in `--release` mode and use the disk in debug and test mode.

All calls to functions in the [assets][assets api] module
should work with the files in your asset directory. Add more and get
to it!

## State

There are two types of state available in **Vial**:

1. Local State - Built-in to [Request]. Allows caching of expensive
   algorithms (like DB lookups) on a per-request basis.

2. Global State - Requires the `state` feature. Allows you to share
   any `Send + Sync + 'static` types (like database connections)
   across all requests.

### Local State

Local state lives for only a single [Request], but can be useful to
prevent looking up the same data over and over. The cache is based on
the return type of the function or closure you pass to `cache()`, so
make sure to create little wrapper structs if you want different
functions to return the same type, like `Vec<String>`:

```rust
struct PageNames(Vec<String>);
struct UserNames(Vec<String>);
```

Here's an example:

```rust
use vial::prelude::*;
use page::Page;
use db;

routes! {
    GET "/" => list;
}

struct PageNames(Vec<String>);

fn all_pages(_: &Request) -> Vec<Page> {
    db::lookup("select * from pages")
}

fn page_names(req: &Request) -> PageNames {
    PageNames(req.cache(all_pages)
        .iter()
        .map(|page| page.name.clone())
        .collect::<Vec<_>>())
}

fn list_of_names(req: &Request) -> String {
    req.cache(page_names)
        .0
        .iter()
        .map(|name| format!("<li>{}</li>", name))
        .collect::<Vec<_>>()
        .join("\n")
}

fn list(req: Request) -> impl Responder {
    format!(
        "<html>
            <head><title>{title}</title></head>
            <body>
                <h1>{title}</h1>
                <h3>There are {page_count} pages:</h3>
                <ul>
                    {pages}
                </ul>
            </body>
        </html>",
        title = "List Pages",
        page_count = req.cache(all_pages).len(),
        pages = req.cache(list_of_names),
    )
}

fn main() {
    run!().unwrap();
}
```

### Global State

There are three steps involved in setting up shared, global state in
**Vial**:

1. Enable the `state` feature in your `Cargo.toml`:

```toml
[Dependencies]
vial = { version = "*", features=["state"] }
```

2. Create a struct that is `Send + Sync` to hold your application's
   shared state:

```rust
use vial;
use std::sync::{Arc, Mutx, atomic::{AtomicUsize, Ordering}};
use some_db_crate::DB;

struct MyConfig {
    db: Arc<Mutex<DB>>,
    counter: AtomicUsize,
}

impl MyConfig {
    pub fn new(db: DB) {
        MyConfig {
            db: Arc::new(Mutex::new(db)),
            counter: AtomicUsize::new(0),
        }
    }
}
```

3. Tell **Vial** about your state object before calling `run!`:

```rust
fn main() {
    let db = DB::new();
    vial::use_state!(MyConfig::new(db));
    vial::run!();
}
```

Now your actions and filters can access `MyConfig` by calling the
[state()](struct.Request.html#method.state) method on [Request]:

```rust
use vial::prelude::*;

routes! {
    GET "/list" => list;
}

fn find_names(db: Arc<Mutex<DB>>) -> Result<Vec<String>, db::Error> {
    Ok(db.lock()?.query("SELECT name FROM names")?
        .map(|row| row.get("name")?)
        .collect::<Vec<_>>())
}

fn list(req: Request) -> Result<String> {
    Ok(
        find_names(req.state::<MyConfig>().db.clone())?
            .map(|name| format!("<li>{}</li>", name))
            .join("\n")
    )
}
```

You might find it more convenient to define and implement a local
trait on the [Request] struct instead of calling `request.state()`
directly:

```rust
use vial::prelude::*;

routes! {
    GET "/list" => list;
}

// ...

trait WithConfig {
    fn config(&self) -> &MyConfig;
}

impl WithConfig for vial::Request {
    fn config(&self) -> &MyConfig {
        self.state::<MyConfig>()
    }
}

fn list(req: Request) -> Result<String> {
    Ok(
        find_names(req.config().db.clone())?
            .map(|name| format!("<li>{}</li>", name))
            .join("\n")
    )
}
```

## Templates

_Optional Feature: Coming soon._

## Cookies

_Optional Feature: Coming soon._

## Sessions

_Optional Feature: Coming soon._

## JSON

_Optional Feature: Coming soon._

## Database

_"Pro Tip": Coming soon._

## Markdown

_"Pro Tip": Coming soon._

[request]: #requests
[response]: #responses
[responder]: #responses
[routing]: #routing
[response api]: https://docs.rs/vial/latest/vial/struct.Response.html
[routes api]: https://docs.rs/vial/latest/vial/macro.routes.html
[asset_dir api]: https://docs.rs/vial/latest/vial/macro.asset_dir.html
[run api]: https://docs.rs/vial/latest/vial/macro.run.html
[assets api]: https://docs.rs/vial/latest/vial/assets/
[etag api]: https://docs.rs/vial/latest/vial/assets/#method.etag
[exists api]: https://docs.rs/vial/latest/vial/assets/#method.exists
[is_bundled api]: https://docs.rs/vial/latest/vial/assets/#method.is_bundled
[to_string api]: https://docs.rs/vial/latest/vial/assets/#method.to_string
[as_reader api]: https://docs.rs/vial/latest/vial/assets/#method.as_reader
