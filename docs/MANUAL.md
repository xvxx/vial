<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# Vial

#### ~ a micro micro-framework ~

**Vial** is a small web "framework" for making small web "sites". It
includes just a handful of basic features for delivering old school,
server-side rendered HTML: request routing, form data parsing,
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

Now here's a bigger bear, showing off most of **Vial**'s features:

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

## Routes

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

1. `"/:name"` — This will match almost anything except paths with `/`
   or `.` in them.
2. `"/:name.md"` — Use this format to match on a specific file extension.
3. `"/*name"` — This will match everything, including `/`.

In the three examples above, calling `req.arg("name")` in an `ACTION`
will deliver `Some(&str)`.

### Actions

`ACTION` can be either of:

1. A closure in the form of `|req| { code }` that returns an
   `impl Responder`
2. The name of a function with the signature of `fn(Request) -> impl Responder`.

### Route Modules

Routes can be defined in different modules and combined together using
the `vial::run!` macro:

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
[Request] object.

### Route Arguments

- `arg(&str) -> Option<&str>`

### Query Parameters

- `query(&str) -> Option<&str>`

### Form Data

- `form(&str) -> Option<&str>`

### Request Headers

- `header(&str) -> Option<&str>`

### Other Info

- `method() -> &str`
- `path() -> &str`

## Responses

All `ACTIONs` return [Responders][responder], which are turned into
[Responses][response] before being sent back to the client.

### HTML from String

### HTML from File

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
