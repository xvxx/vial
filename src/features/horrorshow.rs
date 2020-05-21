use {
    crate::Response,
    horrorshow::{FnRenderer, TemplateBuffer},
    std::fmt,
};

impl<F> From<FnRenderer<F>> for Response
where
    FnRenderer<F>: fmt::Display,
    F: FnOnce(&mut TemplateBuffer<'_>),
{
    fn from(renderer: FnRenderer<F>) -> Response {
        Response {
            body: renderer.to_string(),
            ..Response::default()
        }
    }
}
