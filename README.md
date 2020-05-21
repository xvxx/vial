<img src="./img/drink-me.jpeg" alt="Drink Me." align="left" height="300" />

# ~ vial: a micro micro-framework ~

`vial` is a small web "framework" for making small web "sites".

It only includes the basics:

- Parsing and routing HTTP requests
- Handling POST requests
- Serving static files (css, js)

Everything else... well, that's up to you.

The goal is a small, simple, as-few-as-possible-dependencies web
library you can use to test out an idea quickly or get a static site
_rolling_. Single file, server side apps? You bet! Fast compilation?
Yes please! _À la carte_ dependencies? Now you're talkin'!

It's sort of like a picnic where the playlist is all 90s music and you
have to bring your own beverage. In other words, you're guaranteed to
have a great time.

## ~ hello world ~

As is tradition.

```rust
use vial::vial;

vial! {
    GET "/" => |_| "Hello, world!".into();
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
```

For a bit more sanity, you can route to functions directly:

```rust
use vial::{vial, Request, Response};

vial! {
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> Response {
    Response::from(
        "<form method='POST'>
        <input type='text' name='echo'/>
        <input type='submit'/>
    </form>",
    )
}

fn post(req: Request) -> Response {
    Response::from(format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    ))
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
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
fancy stuff, but there are a few compile-time `--features` you can
activate for enhanced productivity:

<img src="./img/alice.jpeg" alt="Alice" align="right" width="250" />

- [ ] **cookies**: Cookie monster!
- [x] **markdown**: Add Markdown rendering capabilities.
- [x] **horror**: Small & fast macro-based HTML builder, via [horrowshow].
- [ ] **json**: `to_json` and `from_json` powers, via Serde.
- [ ] **tera**: Templating, via Tera.
- [ ] **htxl**: Vial's preferred, no-dependency template library: HTXL.
- [ ] **ssl**: Add support for SSL/TLS. Normally you should be using a
      proxy.

## ~ T0D0 ~

- [x] GET requests
- [x] POST requests
- [ ] route/:recognizer
- [ ] static file
- [ ] static file etag
- [x] parse headers()
- [ ] test headers()
- [ ] test GET param()
- [ ] test POST param()
- [ ] test static file
- [ ] test etag
- [ ] multiple modules
- [ ] `before_filter`
- [ ] `after_filter`

[cargo-watch]: https://crates.io/crates/cargo-watch
[horrowshow]: https://github.com/Stebalien/horrorshow-rs
