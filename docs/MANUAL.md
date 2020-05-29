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

First, here's the bare minimum:

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

- `"/:category/:id/*name"`

This route will populate `request.arg("category")`, `request.arg("id")`,
and `request.arg("name")`.

### Actions

`ACTIONs` are what routes actually route to. Your code. The app.

An `ACTION` can either be a closure or the name of a function:

1. Closures take the form `|req: Request| { code }` and return an
   `impl Responder`.
2. Functions have the signature `fn(Request) -> impl Responder`,
   basically the same thing.

Returning `impl Responder` is easy - [Responder] is a **Vial** trait
that defines a single conversion method:

```rust
pub trait Responder {
    fn to_response(self) -> Response;
}
```

Both `&str` and `String` are `impl Responder`, as well as some other
common types. Making your own is easy, too, because using the
`Response` struct is easy.

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
impl Request {
    fn arg(&self, name: &str) -> Option<&str>;
}
```

### Query Parameters

In addition to route parameters, **Vial** will also parse good ol'
fashioned query string parameters for you:

```rust
impl Request {
    fn query(&self, name: &str) -> Option<&str>;
}
```

For example:

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

### POST Form Data

What's the web without open ended `<textarea>s`? Perish the thought.

POST form data follows the same pattern as query and route parameters:
use `request.form()` to access a form parameter:

```rust
impl Request {
    fn form(&self, name: &str) -> Option<&str>;
}
```

### Request Headers

Headers are available without any of the peksy conveniences of type
safety. Just give `request.header()` a string and hope you get one
back!

```rust
impl Request {
    fn header(&self, name: &str) -> Option<&str>;
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

## Database

## Markdown

[request]: https://docs.rs/vial/latest/vial/struct.Request.html
[response]: https://docs.rs/vial/latest/vial/struct.Response.html
[responder]: https://docs.rs/vial/latest/vial/trait.Responder.html
[routing]: #routing
