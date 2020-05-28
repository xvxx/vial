<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# Vial

#### ~ a micro micro-framework ~

**vial** is a micro web "framework" for making micro web "sites". It
only includes a handful of basic features, hopeful that you'll add
whatever other features you need on your own. Kind of like ice cream
toppings at one of those ice cream places.

The goal is a small, lean core that compiles quickly and has as few
dependencies as possible. Use it for HTML stuff: prototyping ideas,
testing out concepts, or, perhaps, even writing tiny personal apps.

This manual is an overview of **vial**’s built in features, as well as
the few _optional_ features you can enable. It also includes
suggestions for many "common tasks", like using a database to store
information.

## Hello, World

This global greeting shows off some of **vial**'s built-in features.

You can play with it in real time by running this command in your
local copy of this repository:

    $ cargo run --example manual

Feel free to open it in your favorite text editor and poke around!

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
    format!("<h1>Why hello there, {}!</h1>", req.arg("name").unwrap())
}

fn main() {
    run!().unwrap();
}
```

## Getting Started

- Add it to your project
- Include the prelude
- Define `routes!`
- Call `run!`

## Routes

Routes are written using the `routes!` macro in the format:

    HTTP_METHOD ROUTE_PATTERN => ACTION;

The order in which routes are written matters - routes written first
will be checked for matches first, meaning you can declare many routes
that point to `"/"`, but only the first one declared will ever match.

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
   in them or with `.` in them.
2. `"/:name.md"` — Use this format to match on a specific file extension.
3. `"/*name"` — This will match everything, including `/`.

In the three examples above, calling `req.arg("name")` in an `ACTION`
will deliver `Some(&str)`.

### Actions

`ACTION` can be either of:

1. A closure in the form of `|req| { code }` that returns an
   `impl Responder`
2. The name of a function with the signature of `fn(Request) -> impl Responder`.

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
