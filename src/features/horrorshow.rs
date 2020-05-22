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
        Response {
            body: self.to_string(),
            ..Response::default()
        }
    }
}

// impl<F> From<FnRenderer<F>> for Response
// where
//     FnRenderer<F>: fmt::Display,
//     F: FnOnce(&mut TemplateBuffer<'_>),
// {
//     fn from(renderer: FnRenderer<F>) -> Response {
//         Response {
//             body: renderer.to_string(),
//             ..Response::default()
//         }
//     }
// }
