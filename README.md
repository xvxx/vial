# ~ vial: a micro micro-framework ~

`vial` is a small web "framework" for making small web "sites".

It includes but a drop of the bare minimum:

- Parsing and routing HTTP requests
- Handling POST requests
- Serving static files (css, js)

Everything else... well, that's up to you.

The goal is a small, simple, as-few-as-possible-dependencies web
library you can use to test out an idea quickly or get a static site
_rolling_. Single file, server side apps? Yes, please!

It's sort of like a picnic where the playlist is all 90s music and you
have to bring your own beverage. Yabba dabba doo!

## ~ hello world ~

As is tradition... the bare minimum:

```rust
use vial::vial;

vial! {
    GET "/" => |_| "Hello, world!".into();
}

fn main() {
    vial::run!("0.0.0.0:7667");
}
```

For a bit more sanity, you can route to functions directly:

```rust
use vial::{vial, Request, Response};

vial! {
    GET "/hi/world" => |_| "Hello, world!".into();
    GET "/" => echo;
}

fn echo(req: Request) -> Response {
  Response::from(
    format!("You said: <b>{}</b>", req.params("echo").unwrap())
  )
}

fn main() {
    vial::run!("0.0.0.0:7667");
}
```

To _really_ break the mold, you can split your site into different
modules:

```rust
use vial;

mod wiki;
mod blog;

mod index {
    use vial::vial;
    vial! {
        GET "/" => |_| Response::from_file("index.html")
    }
}

fn main() {
    // The order matters here - if `wiki` and `blog` both define "/",
    // the `mod index` version will match first and get run.
    vial::run!("0.0.0.0:7667", index, wiki, blog);
}
```

## ~ hot reloading ~

Install [cargo-watch]:

    $ cargo install cargo-watch
    $ cargo watch -x 'run --example hello_world'

## ~ bonus features ~

**vial** doesn't come with JSON or a template engine or any of that
fancy stuff, but there are a few compile-time features you can
activate for enhanced productivity:

- **cookies**: Cookie monster!
- **markdown**: Add Markdown rendering capabilities.
- **json**: `to_json` and `from_json` powers, via Serde.
- **tera**: Templating, via Tera.
- **htxl**: Vial's preferred, no-dependency template library: HTXL.
- **ssl**: Add support for SSL/TLS. Normally you should be using a
  proxy.

## ~ T0DO ~

- [ ] GET requests
- [ ] POST requests
- [ ] static file
- [ ] static file etag
- [ ] parse headers()
- [ ] test headers()
- [ ] test GET param()
- [ ] test POST param()
- [ ] test static file
- [ ] test etag
- [ ] multiple modules
- [ ] `before_filter`
- [ ] `after_filter`

[cargo-watch]: https://crates.io/crates/cargo-watch
