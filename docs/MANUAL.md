# ~ vial: the manual ~

**vial** is a micro web “framework” for making micro web “sites”. It
includes but a few basic features and was built with the idea that you,
the programmer, will add any functionality you need that isn’t included
by default.

The goal is a small, lean core that compiles quickly and has as few
dependencies as possible - to be used for prototyping ideas,
testing out a new concept, or writing tiny personal apps, perhaps.

This manual will serve as an overview of **vial**’s built in features,
as well as the few _optional_ features you can enable. It will also
include suggestions for many “common tasks”, like using a database
to store information.

## Hello, World

```rust
use vial::prelude::*;

routes! {
    GET "/" => hello_world;
    POST "/" => redirect_to_greeting;
    GET "/:name" => hello_name;
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

- **vial::prelude::\***
  This import...

- **routes!**
  This macro...

- **run!**
  This macro...

- **Request**
  This struct...

- **Request**
  This struct...

- **Responder**
  This trait...

## Routes

- “/blah”
- “/:name”
- “/:name.md”
- “/edit/\*page”
- GET, POST, etc

## Requests

- `query()`
- `arg()`
- `form()`
- `header()`

## Responses

- `Response::from()`
- `Response::from_body()`
- `Response::from_text()`
- `Response::from_file()`
- `Response::from_asset()`
- `Response::from_code()`

## Assets

- `static_dir!`
- `asset::exists()`
- `asset::path()`

## Bundling Assets

- `vial::bundle_assets(path_to_asset_dir)`

## Cookies

## Sessions

## JSON

## State

## Database

## Markdown

## Hatter Templates

## Horrorshow Templates

## Tera Templates
