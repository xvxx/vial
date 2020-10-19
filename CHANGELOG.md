## v{{next_version}}-dev

## v0.1.6

- Fix user agent timeout on empty response body.
- Fix date format in HTTP response.

Thanks to https://redbot.org and @tdryer for this release!

## v0.1.5

- Added optional `json_serde` feature with support for
  JSON via `Request::json` thanks to @tdryer!
- Removed the `state` feature. Global state is built-in.
- Added basic support for [Hatter](https://github.com/xvxx/hatter)
  HTML templates.

## v0.1.4

- Fix routing paths with fewer parts than a pattern.
- Removed the dependency on percent-encoding. Now Vial
  has only **two** direct dependencies and four total.

## v0.1.3

- Hatter now rejects headers that are over 8KB in total.
- Minor changes to HTTP header generation.

## v0.1.2

- Any panic! in app code is now converted into an error page.
- You can now disable or set your own startup banner to show
  in the console.

## v0.1.1

This release fixes a few small bugs in error handling and HTTP
parsing.

## v0.1.0

This is the first public release of **Vial**, a micro micro-framework
for the Rust programming language.

For an overview, please see [the manual][manual] or the [README][readme].

Enjoy.

[manual]: https://vial.rs
[readme]: https://github.com/xvxx/vial#readme
