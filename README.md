# ~ vial: a micro micro-framework ~

`vial` is a small web "framework" for making small web "sites".

It includes but a droplet of the bare minimum:

- Parsing and routing HTTP requests
- Handling POST requests
- Serving static files (css, js)

Everything else... well, that's up to you.

It's sort of like a picnic where the playlist is all 90s music and you
have to bring your own beverage.

Yes, even water.

The goal is a small, simple, as-few-as-possible-dependencies web
library you can use to test out an idea quickly or get a static site
_rolling_.

## ~ bonus features ~

**vial** doesn't come with JSON or a template engine or any of that
fancy stuff, but there are a few compile-time features you can
activate for enhanced productivity:

- **cookies**: Cookie monster!
- **markdown**: Add Markdown rendering capabilities.
- **json**: `to_json` and `from_json` powers, via Serde.
- **tera**: Templating, via Tera.
- **htxl**: Vial's preferred, no-dependency template library:
  HTXL.
- **ssl**: Add support for SSL/TLS. Normally you should be using a
  proxy.
