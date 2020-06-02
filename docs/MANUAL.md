<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# Vial

#### ~ a micro micro-framework ~

**Vial** is a small web "framework" for making small web "sites" in
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

fn hello_world(_req: Request) -> impl Responder {
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

fn hello_name(req: Request) -> impl Responder {
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
directory, so be sure to peruse them skeptically either alongside or
after digesting this manual.

## Getting Started

**Vial** should work on any recent, stable version of **Rust** on
**Linux** or **macOS**.

To begin, add **Vial** to your project's `Cargo.toml`:

```rust
[dependencies]
vial = "*"
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
think about it. In **Vial**, routes are defined with the `routes!`
macro in this format:

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
`ACTION` will return `Some(&str)`.

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

`ACTIONs` are what routes actually route to. Your code. The app.

They are functions or closures take a [Request] and return a
[Responder]:

```rust
use vial::prelude::*;

routes! {
    GET "/info" => |req| format!(
        "<p>Name: {}</p>", req.query("name").unwrap_or("None")
    );
    GET "/" => index;
}

fn index(req: Request) -> impl Responder {
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

Making your own is easy, too, by creating a [Response].

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

When a route matches and an `ACTION` is called, it's passed a
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

### POST Form Data

What's the web without open ended `<textarea>s`? Perish the thought.

POST form data follows the same pattern as query and route parameters:
use `request.form()` to access a form parameter:

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

Every `ACTION` returns an `impl Responder`, which is a trait with a
single method:

```rust
pub trait Responder {
    fn to_response(self) -> Response;
}
```

Common types like `&str` and `Option<String>` already implement this
trait, so you are free to be lazy and return simple objects in your
`ACTIONs`. If, however, you want to set headers and do other fancy
jazz, you'll need to build and return a [Response] object directly.

`Response` objects can be with either a body or status code:

```rust
// Response w/ HTTP Status Code 422, No Body
fn fourtwotwo(_req: Request) -> impl Responder {
    Response::from(422)
}

// Response w/ HTTP Status Code 422 & Body
fn fourtwotwo(_req: Request) -> impl Responder {
    Response::from("404 File Not Found").with_code(404);
}

// Serve the README as HTML. Probably want to Markdownize it first...
fn fourtwotwo(_req: Request) -> impl Responder {
    Response::from_file("README.md");
}
```

### from String

Each `Response` defaults to a `Content-Type` of `text/html; charset=utf8`, so you can build HTML with your bare hands and **Vial**
will lovingly deliver it to the client:

```rust
fn index(_req: Request) -> impl Responder {
    Response::from("<marquee>Coming soon!</marquee>")
}
```

### from Asset

### Redirect

### Status Codes

### Headers

## Assets

### Setting asset dir

### `asset::exists()`

### `asset::path()`

### Bundling Assets

- `vial::bundle_assets(path_to_asset_dir)`

## Templates

### Hatter

### Horrorshow

### Tera

## Cookies

## Sessions

## JSON

## State

There are two types of state available in **Vial**:

1. Local State - Built-in to [Request]. Allows caching of expensive
   algorithms (like DB lookups) on a per-request basis.

2. Global State - Requires the `stateful` feature. Allows you to share
   database connections and whatnot across all requests.

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

Here's a longer winded example:

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

There are four steps involved in setting up shared, global state in
**Vial**:

1. Enable the `stateful` feature in your `Cargo.toml`:

```toml
[Dependencies]
vial = { version = "*", features=["stateful"] }
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

3. Write your actions to take `State<MyConfig>` instead of `Request`:

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

fn list(state: State<MyConfig>) -> VialResult {
    Ok(find_names(state.db.clone())?.map(|name| format!("<li>{}</li>", name)))
}
```

4. Tell **Vial** about your state object before calling `run!`:

```rust
fn main() {
    let db = DB::new();
    vial::use_state!(MyConfig::new(db));
    vial::run!();
}
```

## Database

## Markdown

[request]: https://docs.rs/vial/latest/vial/struct.Request.html
[response]: https://docs.rs/vial/latest/vial/struct.Response.html
[responder]: https://docs.rs/vial/latest/vial/trait.Responder.html
[routing]: #routing

```

```
