# ~ vial: the manual ~

**vial** is a micro web "framework" for making micro web "sites". It
includes but a few basic features and was built under the assumption
that you, dear programmer, will add any functionality you need to your
individual app.

The goal is a small, lean core that compiles quickly and has as few
dependencies as possible.

This manual will serve as an overview of the features that **vial**
does have, including the few _optional_ features you can enable. It
will also include suggestions for many "common tasks", like using a
database to store information.

## Hello, World

- prelude
- routes!
- run!
- Request
- Responder

## Routes

- "/blah"
- "/:name"
- "/:name.md"
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

## Static Files

- `static_dir!`
- `asset::exists()`
- `asset::path()`
- `asset::bundle()`

## Cookies

## Sessions

## JSON

## State

## Database

## Markdown

## Hatter Templates

## Horrorshow Templates

## Tera Templates

