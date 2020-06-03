<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# ~ vial: a micro micro-framework ~

**Vial** is a small web "framework" for making small web sites.

It only includes a few basics:

- Parsing and routing HTTP requests
- Handling POST requests
- Serving static files (css, js)

Everything else... well, that's up to you.

The goal is an as-few-as-possible-dependencies web library you can
use to test out an idea quickly or get a personal project _rolling_.
Single file, server side apps? You bet! Fast compilation? Yes please!
_À la carte_ dependencies? Now you're talkin'!

It's sort of like a picnic where the playlist is all 90s music and you
have to bring your own beverages. And food.

----

**Status:** Vial is currently in early development. It is being
developed alongside [deadwiki], but that is _strictly_ for personal
use. Proceed with caution.

---

To get started, just add `vial` to your `Cargo.toml`:

```toml
[dependencies]
vial = "*"
```

## ~ hello world ~

As is tradition:

```rust
vial::routes! {
    GET "/" => |_| "Hello, world!";
}

fn main() {
    vial::run!().unwrap();
}
```

For a bit more sanity, you can route to functions directly:

```rust
use vial::prelude::*;

routes! {
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> &'static str {
    "<form method='POST'>
        <input type='text' name='echo'/>
        <input type='submit'/>
    </form>"
}

fn post(req: Request) -> String {
    format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    )
}

fn main() {
    vial::run!().unwrap();
}
```

To _really_ break the mold, you can split your site into different
modules:

```rust
use vial;

mod wiki;
mod blog;

mod index {
    use vial::prelude::*;
    routes! {
        GET "/" => |_| Response::from_file("index.html")
    }
}

fn main() {
    // The order matters here - if `wiki` and `blog` both define "/",
    // the `mod index` version will match first and get run.
    vial::run!(index, wiki, blog);
}
```

But hey, who wants to putz around with HTML when you can be writing
**Rust**? Enable the `horror` feature and you're on your way:

```rust
use vial::prelude::*;

routes! {
    GET "/" => |_| html! {
        p {
            : "You're looking for this: ";
            a(href="/echo") { : "echo" }
        }
    };
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> impl Responder {
    html! {
        form(method="POST") {
            p {
            : "Type something: ";
                input(type="text", name="echo");
                input(type="submit");
            }
        }
    }
}

fn post(req: Request) -> impl Responder {
    owned_html! {
        h1: req.form("echo")
            .unwrap_or("You didn't say anything!");
    }
}

fn main() {
    vial::run!().unwrap();
}
```

## ~ hot reloading ~

Install [cargo-watch]:

    $ cargo install cargo-watch
    $ cargo watch -x 'run --example hello_world'

## ~ bonus features ~

**vial** doesn't come with JSON or a template engine or any of that
fancy stuff by default, but there are a few compile-time `--features`
you can activate for enhanced productivity:

<img src="./img/alice.jpeg" alt="Alice" align="right" width="250" />

- [x] **state**: Global State
- [x] **horror**: Small & fast macro-based HTML builder, via [horrowshow].
- [ ] **cookies**: Cookie monster!
- [ ] **sessions**: Session support
- [ ] **multipart**: Multipart form data (file uploads)
- [ ] **log**: Access logging
- [ ] **json**: `to_json` and `from_json` powers, via Serde.

_**Please note:** The list above is a work-in-progress._

## ~ T0D0 ~

- [ ] tests
- [ ] `before_filter`

## ~ testing ~

Tests can be found in `tests/main.rs`. You can run them on a recent
version of stable Rust with `make test`.

## ~ license ~

**Vial** is licensed under either of the following, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[cargo-watch]: https://crates.io/crates/cargo-watch
[horrowshow]: https://github.com/Stebalien/horrorshow-rs
[deadwiki]: https://github.com/xvxx/deadwiki
