use {
    crate::{Responder, Response},
    horrorshow::{FnRenderer, TemplateBuffer},
    std::fmt,
};

impl<F> Responder for FnRenderer<F>
where
    FnRenderer<F>: fmt::Display,
    F: FnOnce(&mut TemplateBuffer<'_>),
{
    fn to_response(self) -> Response {
        Response::from_body(self.to_string())
    }
}
